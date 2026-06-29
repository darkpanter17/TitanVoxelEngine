#[allow(unused_imports)]
use image::GenericImageView;

pub struct Texture {
    #[allow(dead_code)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    // Create the Z-Buffer (Depth)
    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0, lod_max_clamp: 100.0,
            ..Default::default()
        });
        Self { texture, view, sampler }
    }

    // LOAD TEXTURE ARRAY (The difficult part)
    pub fn load_texture_array(device: &wgpu::Device, queue: &wgpu::Queue, paths: Vec<String>) -> anyhow::Result<Self> {
        let mut layers = Vec::new();
        let (mut width, mut height) = (0, 0);

        // 1. Load images and ensure size matches
        for (i, path) in paths.iter().enumerate() {
            let img = image::open(path)?.to_rgba8();
            let dim = img.dimensions();
            
            if i == 0 {
                width = dim.0; height = dim.1;
            } else {
                // In a real engine, we would resize here. For now we panic if they don't match.
                assert_eq!(dim.0, width, "All textures must have the same width");
                assert_eq!(dim.1, height, "All textures must have the same height");
            }
            layers.push(img);
        }

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: layers.len() as u32, // N Layers
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 2. Copy bytes to the GPU layer by layer
        for (i, img) in layers.iter().enumerate() {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: i as u32 }, // Z is the array index
                    aspect: wgpu::TextureAspect::All,
                },
                img,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Texture Array View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array), // IMPORTANT!
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest, // Pixel Art look
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self { texture, view, sampler })
    }

    /// Creates a placeholder texture array (1x1 per layer) when download or load fails.
    pub fn create_placeholder_texture_array(device: &wgpu::Device, queue: &wgpu::Queue, num_layers: u32) -> Self {
        let width = 1u32;
        let height = 1u32;
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: num_layers,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Placeholder Texture Array"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // Fill each layer with a different color (RGBA 1x1) to distinguish blocks
        let colors: [[u8; 4]; 3] = [
            [100, 70, 50, 255],   // brown (dirt)
            [80, 140, 60, 255],   // green (grass)
            [120, 120, 120, 255], // gray (stone)
        ];
        for i in 0..num_layers.min(3) {
            let c = colors[i as usize];
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: i },
                    aspect: wgpu::TextureAspect::All,
                },
                &c,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            );
        }
        for i in 3..num_layers {
            let c: [u8; 4] = [80, 80, 80, 255];
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: i },
                    aspect: wgpu::TextureAspect::All,
                },
                &c,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            );
        }
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Placeholder Texture Array View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        Self { texture, view, sampler }
    }
}