use nethack_core::GameState;
use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Window;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;
    let mut app = NetHackApp::new();

    event_loop.run_app(&mut app)?;
    Ok(())
}

struct NetHackApp {
    window: Option<Window>,
    game_state: GameState,
}

impl NetHackApp {
    fn new() -> Self {
        Self {
            window: None,
            game_state: GameState::new(),
        }
    }
}

impl ApplicationHandler for NetHackApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("NetHack")
                        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0)),
                )
                .unwrap();
            tracing::info!("NetHack initialized at depth {}", self.game_state.depth);
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
        // TODO: Handle window events
    }
}
