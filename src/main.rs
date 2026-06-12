mod chunk; mod mesher; mod texture; mod shader_loader; mod state; mod camera;

use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};
use state::State;
use mlua::prelude::*;
use std::fs::{self, File};

fn main() {
    env_logger::init();
    
    // --- LUA SETUP ---
    let lua = Lua::new();
    // Cliente HTTP con User-Agent (GitHub y otros exigen UA; sin él devuelven HTML)
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitanVoxelEngine/1.0")
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());

    // Exponer función de descarga a Lua (Bloqueante para el init)
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
        // Validar firma PNG (89 50 4E 47 0D 0A 1A 0A) para no guardar HTML/error
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

    // Ejecutar Downloader
    if let Ok(script) = fs::read_to_string("scripts/asset_downloader.lua") {
        if let Err(e) = lua.load(&script).exec() {
            eprintln!("Error en script Lua: {}", e);
        }
    }

    // --- GRAPHICS SETUP ---
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("Titan Voxel: Textured").build(&event_loop).unwrap();
    window.set_cursor_visible(false); // Ocultar ratón para modo FPS

    let mut state = match pollster::block_on(State::new(&window)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize wgpu state: {}", e);
            return;
        }
    };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.window.id() => {
                if !state.input(event) { 
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput {
                            event: winit::event::KeyEvent {
                                physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                            ..
                        } => elwt.exit(),
                        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta, }, .. } => {
                state.input_mouse(delta.0, delta.1);
            }
            Event::AboutToWait => state.window.request_redraw(),
            _ => {}
        }
    }).unwrap();
}