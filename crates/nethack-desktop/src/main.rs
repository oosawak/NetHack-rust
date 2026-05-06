use nethack_core::GameBridge;
use nethack_render::{WgpuRenderer, Vertex};
use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::window::Window;
use std::sync::Arc;
use wgpu::util::DeviceExt;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;
    let mut app = NetHackApp::new();

    event_loop.run_app(&mut app)?;
    Ok(())
}

struct NetHackApp {
    window: Option<Arc<Window>>,
    game_bridge: Option<GameBridge>,
    renderer: Option<WgpuRenderer>,
    vertex_buffer: Option<wgpu::Buffer>,
    device: Option<Arc<wgpu::Device>>,
    running: bool,
}

impl NetHackApp {
    fn new() -> Self {
        Self {
            window: None,
            game_bridge: None,
            renderer: None,
            vertex_buffer: None,
            device: None,
            running: true,
        }
    }

    fn init_graphics(&mut self) -> anyhow::Result<()> {
        if let Some(window) = &self.window {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });

            let surface = instance.create_surface(window.clone())?;
            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable GPU adapter"))?;

            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            ))?;

            let size = window.inner_size();
            let surface_config = surface
                .get_default_config(&adapter, size.width, size.height)
                .ok_or_else(|| anyhow::anyhow!("Surface not supported by adapter"))?;
            surface.configure(&device, &surface_config);

            let device_arc = Arc::new(device);
            let queue_arc = Arc::new(queue);
            let surface_arc = Arc::new(surface);

            let renderer = pollster::block_on(WgpuRenderer::new(
                device_arc.clone(),
                queue_arc.clone(),
                surface_arc,
                surface_config,
            ))
            .map_err(|e| anyhow::anyhow!("Failed to create renderer: {}", e))?;

            // Create a simple test triangle
            let vertices = [
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                },
            ];

            let vertex_buffer = device_arc.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            self.renderer = Some(renderer);
            self.vertex_buffer = Some(vertex_buffer);
            self.device = Some(device_arc);

            tracing::info!("Graphics initialized successfully");
        }

        Ok(())
    }

    fn init_game(&mut self) {
        let mut bridge = GameBridge::new();
        
        match bridge.init_game() {
            Ok(_) => {
                tracing::info!("Game initialized successfully");
                tracing::info!("Dungeon level: {}", bridge.dungeon_level());
                self.game_bridge = Some(bridge);
            }
            Err(e) => {
                tracing::error!("Failed to initialize game: {}", e);
            }
        }
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowUp => {
                tracing::info!("Player moved up");
            }
            KeyCode::ArrowDown => {
                tracing::info!("Player moved down");
            }
            KeyCode::ArrowLeft => {
                tracing::info!("Player moved left");
            }
            KeyCode::ArrowRight => {
                tracing::info!("Player moved right");
            }
            KeyCode::KeyV => {
                // Cycle view modes
                tracing::info!("Switching view mode");
            }
            KeyCode::KeyQ => {
                tracing::info!("Quitting game");
                self.running = false;
            }
            _ => {}
        }
    }

    fn render(&self) -> anyhow::Result<()> {
        if let (Some(renderer), Some(vertex_buffer)) = (&self.renderer, &self.vertex_buffer) {
            renderer.render(vertex_buffer, 3)?;
        }
        Ok(())
    }
}

impl ApplicationHandler for NetHackApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("NetHack - FFI First Rust/WASM")
                        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 960.0)),
                )
                .unwrap();
            
            tracing::info!("Window created: 1280x960");
            self.window = Some(Arc::new(window));
            
            // Initialize graphics
            if let Err(e) = self.init_graphics() {
                tracing::error!("Failed to initialize graphics: {}", e);
            }

            // Initialize the game
            self.init_game();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                        self.handle_input(code);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.render() {
                    tracing::error!("Render error: {}", e);
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CloseRequested => {
                tracing::info!("Close requested");
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Game loop timing
    }
}
