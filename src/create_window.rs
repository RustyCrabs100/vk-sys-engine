// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use winit::{
        application::ApplicationHandler,
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop},
        window::{self, Window, WindowAttributes, WindowButtons},
    };

    /// Handles Keyboard and Mouse Inputs (Currently Unavaliable)
    pub mod input_handler {

        /// Handles Keyboard Input (Stub Function)
        pub fn keyboard_input_handler() {}

        /// Handles Mouse Input (Stub Function)
        pub fn mouse_input_handler() {}
    }

    #[derive(Default)]
    pub struct AppWindow {
        window: Option<Window>,
    }

    impl AppWindow {
        pub fn create_event_loop() -> EventLoop<()> {
            let event_loop = EventLoop::new().unwrap();

            event_loop.set_control_flow(ControlFlow::Poll);

            event_loop
        }

        pub fn run_engine_window(event_loop: EventLoop<()>) {
            let mut app = AppWindow::default();
            event_loop.run_app(&mut app).unwrap();
        }
    }

    impl ApplicationHandler for AppWindow {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            self.window = Some(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: window::WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Closed Requested; Stopping Program");
                    event_loop.exit();
                }
                WindowEvent::Destroyed => println!("Window has been Destroyed"),
                WindowEvent::RedrawRequested => {
                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    }
}
