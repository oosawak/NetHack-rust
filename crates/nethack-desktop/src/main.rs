use nethack_core::GameBridge;
use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::window::Window;
use std::sync::Arc;

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
    running: bool,
}

impl NetHackApp {
    fn new() -> Self {
        Self {
            window: None,
            game_bridge: None,
            running: true,
        }
    }

    fn init_game(&mut self) {
        // TODO: Initialize game only when needed
        // For now, defer to later implementation
        tracing::info!("Game initialization deferred");
        
        // Uncomment when ready for full FFI integration:
        /*
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
        */
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
        use winit::keyboard::Key;

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    // Extract KeyCode from the physical_key
                    if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                        self.handle_input(code);
                    }
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
        if !self.running {
            // Game loop can be updated here
        }
    }
}
