use winit::{event::*, window::Window};
use wgpu::util::DeviceExt;
use crate::{mesher::{self, Vertex}, chunk::Chunk, texture, camera};
use glam::{IVec3, Vec3};

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>, pub device: wgpu::Device, pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration, pub size: winit::dpi::PhysicalSize<u32>, pub window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, num_indices: u32,
    depth_texture: texture::Texture,
    camera: camera::Camera, camera_controller: camera::CameraController, camera_uniform: camera::CameraUniform, camera_buffer: wgpu::Buffer, camera_bind_group: wgpu::BindGroup,
    // NUEVO: Grupo de texturas
    diffuse_bind_group: wgpu::BindGroup, 
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..Default::default() });
        let surface = instance.create_surface(window)?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.ok_or_else(|| anyhow::anyhow!("Failed to request adapter"))?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor { label: Some("Device"), required_features: wgpu::Features::TEXTURE_BINDING_ARRAY | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING, required_limits: wgpu::Limits::default() }, None).await?;
        let config = surface.get_default_config(&adapter, size.width, size.height).ok_or_else(|| anyhow::anyhow!("Failed to get default surface config"))?;
        surface.configure(&device, &config);

        // --- LOAD TEXTURES ---
        // Buscamos las texturas bajadas por Lua
        let texture_paths = vec![
            "assets/textures/1.png".to_string(), // Dirt
            "assets/textures/2.png".to_string(), // Grass
            "assets/textures/3.png".to_string(), // Stone
        ];
        
        // Cargar array; si falla (URLs caídas o no PNG), usar placeholder para que la ventana abra
        let texture_array = match texture::Texture::load_texture_array(&device, &queue, texture_paths.clone()) {
            Ok(t) => t,
            Err(e) => {
                log::warn!("Texturas no cargadas: {}. Usando placeholder.", e);
                texture::Texture::create_placeholder_texture_array(&device, &queue, texture_paths.len() as u32)
            }
        };

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2Array, sample_type: wgpu::TextureSampleType::Float { filterable: true } },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture_array.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&texture_array.sampler) },
            ],
            label: Some("diffuse_bind_group"),
        });

        // --- CAMERA ---
        let camera = camera::Camera { eye: Vec3::new(-5.0, 10.0, -5.0), target: Vec3::new(1.0, -0.5, 1.0).normalize(), up: Vec3::Y, aspect: config.width as f32 / config.height as f32, fovy: 45.0f32.to_radians(), znear: 0.1, zfar: 1000.0, yaw: -45.0f32.to_radians(), pitch: -20.0f32.to_radians() };
        let mut camera_uniform = camera::CameraUniform::new(); camera_uniform.update_view_proj(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Camera Buffer"), contents: bytemuck::cast_slice(&[camera_uniform]), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST });
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }], label: Some("camera_bind_group_layout") });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &camera_bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() }], label: Some("camera_bind_group") });
        let camera_controller = camera::CameraController::new(0.2, 0.003);

        // --- PIPELINE ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("Shader"), source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/voxel.wgsl").into()) });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("Render Pipeline Layout"), 
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout], // 2 Bind Groups
            push_constant_ranges: &[] 
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::desc()] },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })] }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: Some(wgpu::Face::Back), ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: texture::Texture::DEPTH_FORMAT, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() }),
            multisample: wgpu::MultisampleState::default(), multiview: None,
        });

        // --- CHUNK DATA ---
        let mut chunk = Chunk::new(IVec3::ZERO);
        for x in 0..32 { for z in 0..32 { for y in 0..16 {
            let id = if y == 15 { 1 } else { 2 }; // 1=Grass (Top), 2=Dirt (Bottom)
            chunk.set_voxel(x, y, z, id);
        }}}
        let mesh = mesher::generate_mesh(&chunk);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Vertex Buffer"), contents: bytemuck::cast_slice(&mesh.vertices), usage: wgpu::BufferUsages::VERTEX });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("Index Buffer"), contents: bytemuck::cast_slice(&mesh.indices), usage: wgpu::BufferUsages::INDEX });
        let num_indices = mesh.indices.len() as u32;
        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        Ok(Self { window, surface, device, queue, config, size, render_pipeline, vertex_buffer, index_buffer, num_indices, depth_texture, camera, camera_controller, camera_uniform, camera_buffer, camera_bind_group, diffuse_bind_group })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size; self.config.width = new_size.width; self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool { self.camera_controller.process_events(event) }
    pub fn input_mouse(&mut self, dx: f64, dy: f64) { self.camera_controller.process_mouse(dx, dy, &mut self.camera); }
    pub fn update(&mut self) { self.camera_controller.update_camera(&mut self.camera); self.camera_uniform.update_view_proj(&self.camera); self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform])); }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?; let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.5, g: 0.7, b: 0.9, a: 1.0 }), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_texture.view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default()
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.diffuse_bind_group, &[]); // ¡TEXTURAS!
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish())); output.present(); Ok(())
    }
}