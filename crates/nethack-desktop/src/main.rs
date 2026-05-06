use nethack_core::{GameBridge, GameRenderer};
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
    game_renderer: Option<GameRenderer>,
    wgpu_renderer: Option<WgpuRenderer>,
    vertex_buffer: Option<wgpu::Buffer>,
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    running: bool,
    frame_count: u32,
}

impl NetHackApp {
    fn new() -> Self {
        Self {
            window: None,
            game_bridge: None,
            game_renderer: None,
            wgpu_renderer: None,
            vertex_buffer: None,
            device: None,
            queue: None,
            running: true,
            frame_count: 0,
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

            // Create initial vertex buffer (will be updated each frame)
            let initial_vertices: Vec<Vertex> = vec![];
            let vertex_buffer = if !initial_vertices.is_empty() {
                device_arc.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&initial_vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                })
            } else {
                // Create empty buffer that we'll update later
                device_arc.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Vertex Buffer"),
                    size: 65536, // 64KB buffer for vertex data
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            };

            self.wgpu_renderer = Some(renderer);
            self.vertex_buffer = Some(vertex_buffer);
            self.device = Some(device_arc);
            self.queue = Some(queue_arc);
            self.game_renderer = Some(GameRenderer::new());

            tracing::info!("Graphics initialized successfully");
        }

        Ok(())
    }

    fn init_game(&mut self) {
        // NOTE: Full game initialization is deferred due to C linking issues
        // For now, we use default player position and simple game state
        self.game_bridge = Some(GameBridge::new());
        tracing::info!("Game state initialized (simplified mode)");
    }

    fn update_game_state(&mut self) {
        // Update game renderer with current game state
        if let Some(game_renderer) = &mut self.game_renderer {
            // Use default player position (0, 0) for rendering
            // In full implementation, this would read from C game state
            let player_x = 40; // Center of 80-width dungeon
            let player_y = 12; // Center of 24-height dungeon

            // Update renderer vertices
            game_renderer.update_from_game_state(
                player_x,
                player_y,
                80, // dungeon width (standard NetHack)
                24, // dungeon height
            );

            tracing::debug!("Updated game state: player at ({}, {}), {} vertices", 
                player_x, player_y, game_renderer.vertex_count());
        }
    }

    fn update_vertex_buffer(&mut self) {
        if let (Some(game_renderer), Some(queue), Some(vbuf)) = 
            (&self.game_renderer, &self.queue, &self.vertex_buffer) {
            
            let vertices = game_renderer.vertices();
            if !vertices.is_empty() {
                // Convert RenderVertex to Vertex (same structure with #[repr(C)])
                let vertex_data: Vec<u8> = bytemuck::cast_slice(vertices).to_vec();
                queue.write_buffer(vbuf, 0, &vertex_data);
            }
        }
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowUp => {
                tracing::info!("Move up");
            }
            KeyCode::ArrowDown => {
                tracing::info!("Move down");
            }
            KeyCode::ArrowLeft => {
                tracing::info!("Move left");
            }
            KeyCode::ArrowRight => {
                tracing::info!("Move right");
            }
            KeyCode::KeyV => {
                tracing::info!("Switch view mode");
            }
            KeyCode::KeyQ => {
                tracing::info!("Quit");
                self.running = false;
            }
            _ => {}
        }
    }

    fn render(&self) -> anyhow::Result<()> {
        if let (Some(renderer), Some(game_renderer), Some(vbuf)) = 
            (&self.wgpu_renderer, &self.game_renderer, &self.vertex_buffer) {
            
            if game_renderer.vertex_count() > 0 {
                renderer.render(vbuf, game_renderer.vertex_count())?;
            }
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

            // Initial game state update
            self.update_game_state();
            self.update_vertex_buffer();
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
                // Update game state each frame
                self.update_game_state();
                self.update_vertex_buffer();

                // Render frame
                if let Err(e) = self.render() {
                    tracing::error!("Render error: {}", e);
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }

                self.frame_count += 1;
                if self.frame_count % 60 == 0 {
                    tracing::info!("Frame: {}", self.frame_count);
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
        // Game loop timing - request redraw to keep rendering
        if self.running {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}
