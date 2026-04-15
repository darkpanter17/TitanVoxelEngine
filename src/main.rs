mod chunk; mod mesher; mod texture; mod shader_loader; mod state; mod camera;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use state::State;
use mlua::prelude::*;
use std::fs::{self, File};
use std::sync::Arc;

struct App<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes().with_title("Titan Voxel: Textured")).unwrap());
            // pollster block_on is not recommended for production in resumed, but we keep it simple here to match the current async flow
            let state = pollster::block_on(State::new(window));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let Some(state) = self.state.as_mut() {
            if !state.input(&event) {
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: DeviceEvent) {
         if let Some(state) = self.state.as_mut() {
             if let DeviceEvent::MouseMotion { delta } = event {
                 state.input_mouse(delta.0, delta.1);
             }
         }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_ref() {
            state.window.request_redraw();
        }
    }
}

fn main() {
    env_logger::init();
    
    // --- LUA SETUP ---
    let lua = Lua::new();
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitanVoxelEngine/1.0")
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());

    let download_fn = lua.create_function(move |_, (url, path): (String, String)| {
        println!("[Rust] Descargando: {} -> {}", url, path);
        if let Some(parent) = std::path::Path::new(&path).parent() {
            fs::create_dir_all(parent).ok();
        }
        let response = match client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return Ok(false),
        };
        if !response.status().is_success() {
            eprintln!("[Rust] HTTP {} para {}", response.status(), url);
            return Ok(false);
        }
        let bytes = match response.bytes() {
            Ok(b) => b,
            Err(_) => return Ok(false),
        };
        const PNG_SIG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if bytes.len() < 8 || bytes[0..8] != PNG_SIG {
            eprintln!("[Rust] No es PNG válido ({} bytes)", bytes.len());
            return Ok(false);
        }
        match File::create(&path).and_then(|mut f| std::io::Write::write_all(&mut f, &bytes)) {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("[Rust] Error escribiendo {}: {}", path, e);
                Ok(false)
            }
        }
    }).unwrap();
    
    lua.globals().set("download_file", download_fn).unwrap();

    if let Ok(script) = fs::read_to_string("scripts/asset_downloader.lua") {
        if let Err(e) = lua.load(&script).exec() {
            eprintln!("Error en script Lua: {}", e);
        }
    }

    // --- GRAPHICS SETUP ---
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App { state: None };
    event_loop.run_app(&mut app).unwrap();
}
