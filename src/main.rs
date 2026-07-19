mod chunk; mod mesher; mod texture; mod shader_loader; mod state; mod camera;

use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};
use winit::keyboard::{KeyCode, PhysicalKey};
use state::State;
use mlua::prelude::*;
use std::fs::{self, File};

fn main() {
    env_logger::init();
    
    // --- LUA SETUP ---
    let lua = Lua::new();
    // HTTP Client with User-Agent (GitHub and others require UA; otherwise they return HTML)
    let client = reqwest::blocking::Client::builder()
        .user_agent("TitanVoxelEngine/1.0")
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());

    // Expose download function to Lua (Blocking for init)
    let download_fn = lua.create_function(move |_, (url, path): (String, String)| {
        println!("[Rust] Downloading: {} -> {}", url, path);
        
        if let Some(parent) = std::path::Path::new(&path).parent() {
            fs::create_dir_all(parent).ok();
        }

        let response = match client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return Ok(false),
        };
        if !response.status().is_success() {
            eprintln!("[Rust] HTTP {} for {}", response.status(), url);
            return Ok(false);
        }
        let bytes = match response.bytes() {
            Ok(b) => b,
            Err(_) => return Ok(false),
        };
        // Validate PNG signature (89 50 4E 47 0D 0A 1A 0A) to avoid saving HTML/errors
        const PNG_SIG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if bytes.len() < 8 || bytes[0..8] != PNG_SIG {
            eprintln!("[Rust] Invalid PNG ({} bytes)", bytes.len());
            return Ok(false);
        }
        match File::create(&path).and_then(|mut f| std::io::Write::write_all(&mut f, &bytes)) {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("[Rust] Error writing {}: {}", path, e);
                Ok(false)
            }
        }
    }).unwrap();
    
    lua.globals().set("download_file", download_fn).unwrap();

    // Execute Downloader
    if let Ok(script) = fs::read_to_string("scripts/asset_downloader.lua") {
        if let Err(e) = lua.load(&script).exec() {
            eprintln!("Lua script error: {}", e);
        }
    }

    // --- GRAPHICS SETUP ---
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("Titan Voxel: Textured").build(&event_loop).unwrap();
    window.set_cursor_visible(false); // Hide cursor for FPS mode

    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.window.id() => {
                if !state.input(event) { 
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput {
                            event: KeyEvent { state: ElementState::Pressed, physical_key: PhysicalKey::Code(KeyCode::Escape), .. },
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