use nethack_core::{GameBridge, GameRenderer, Camera3D, ViewMode};
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
    camera: Option<Camera3D>,
    wgpu_renderer: Option<WgpuRenderer>,
    vertex_buffer: Option<wgpu::Buffer>,
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    window_width: u32,
    window_height: u32,
    running: bool,
    frame_count: u32,
    current_view_mode: ViewMode,
}

impl NetHackApp {
    fn new() -> Self {
        Self {
            window: None,
            game_bridge: None,
            game_renderer: None,
            camera: None,
            wgpu_renderer: None,
            vertex_buffer: None,
            device: None,
            queue: None,
            window_width: 1280,
            window_height: 960,
            running: true,
            frame_count: 0,
            current_view_mode: ViewMode::Isometric,
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
            self.window_width = size.width;
            self.window_height = size.height;

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

            let initial_vertices: Vec<Vertex> = vec![];
            let vertex_buffer = if !initial_vertices.is_empty() {
                device_arc.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&initial_vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                })
            } else {
                device_arc.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Vertex Buffer"),
                    size: 65536,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            };

            self.wgpu_renderer = Some(renderer);
            self.vertex_buffer = Some(vertex_buffer);
            self.device = Some(device_arc);
            self.queue = Some(queue_arc);
            self.game_renderer = Some(GameRenderer::new());
            self.camera = Some(Camera3D::new(40.0, 0.0, 12.0, self.current_view_mode));

            tracing::info!("Graphics initialized ({}x{})", self.window_width, self.window_height);
        }

        Ok(())
    }

    fn init_game(&mut self) {
        self.game_bridge = Some(GameBridge::new());
        tracing::info!("Game state initialized");
    }

    fn update_game_state(&mut self) {
        if let Some(game_renderer) = &mut self.game_renderer {
            let player_x = 40;
            let player_y = 12;

            game_renderer.update_from_game_state(player_x, player_y, 80, 24);

            if let Some(camera) = &mut self.camera {
                camera.follow(player_x as f32, 0.0, player_y as f32);
            }
        }
    }

    fn update_vertex_buffer(&mut self) {
        if let (Some(game_renderer), Some(queue), Some(vbuf)) = 
            (&self.game_renderer, &self.queue, &self.vertex_buffer) {
            
            let vertices = game_renderer.vertices();
            if !vertices.is_empty() {
                let vertex_data: Vec<u8> = bytemuck::cast_slice(vertices).to_vec();
                queue.write_buffer(vbuf, 0, &vertex_data);
            }
        }
    }

    fn update_camera_uniform(&mut self) {
        if let (Some(camera), Some(renderer)) = (&self.camera, &mut self.wgpu_renderer) {
            let aspect = self.window_width as f32 / self.window_height as f32;
            let view_proj = camera.view_projection(aspect);
            renderer.update_camera(view_proj.to_cols_array_2d());

            tracing::debug!("Camera: {:?}, aspect={:.2}", camera.mode, aspect);
        }
    }

    fn switch_view_mode(&mut self, mode: ViewMode) {
        self.current_view_mode = mode;
        if let Some(camera) = &mut self.camera {
            camera.switch_mode(mode);
            tracing::info!("View mode: {:?}", mode);
        }
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Digit1 => self.switch_view_mode(ViewMode::TopDown),
            KeyCode::Digit2 => self.switch_view_mode(ViewMode::Isometric),
            KeyCode::Digit3 => self.switch_view_mode(ViewMode::FirstPerson),
            KeyCode::Digit4 => self.switch_view_mode(ViewMode::ThirdPerson),
            KeyCode::Digit5 => self.switch_view_mode(ViewMode::Cinematic),
            KeyCode::ArrowUp => tracing::debug!("Move up"),
            KeyCode::ArrowDown => tracing::debug!("Move down"),
            KeyCode::ArrowLeft => tracing::debug!("Move left"),
            KeyCode::ArrowRight => tracing::debug!("Move right"),
            KeyCode::KeyV => tracing::info!("View toggle"),
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
                        .with_title("NetHack - FFI First Rust/WASM (Phase 4.3 Camera)")
                        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 960.0)),
                )
                .unwrap();
            
            self.window = Some(Arc::new(window));
            
            if let Err(e) = self.init_graphics() {
                tracing::error!("Graphics init failed: {}", e);
            }

            self.init_game();
            self.update_game_state();
            self.update_vertex_buffer();
            self.update_camera_uniform();
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
                self.update_game_state();
                self.update_vertex_buffer();
                self.update_camera_uniform();

                if let Err(e) = self.render() {
                    tracing::error!("Render error: {}", e);
                }
                
                if let Some(window) = &self.window {
                    window.request_redraw();
                }

                self.frame_count += 1;
                if self.frame_count % 300 == 0 {
                    tracing::info!("Frame {} | View: {:?}", self.frame_count, self.current_view_mode);
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.running {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}
