// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use winit::{
        application::ApplicationHandler, dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop}, keyboard::KeyCode, window::{self, Window, WindowAttributes}
    };
    use std::{io::Error, sync::{
        Arc, Mutex
    }};
    use smol::block_on;

    /// Handles Keyboard and Mouse Inputs (Currently Unavaliable)
    pub mod input_handler {

        /// Handles Keyboard Input (Stub Function)
        pub fn keyboard_input_handler() {}

        /// Handles Mouse Input (Stub Function)
        pub fn mouse_input_handler() {}
    }

    #[derive(Default)]
    pub struct AppWindow {
        pub window: Arc<Mutex<Option<Window>>>,
        pub window_attr: WindowAttributes,
        pub initalized_window: bool,
    }

    impl AppWindow {
        pub fn new(
            title: &str,
            width: u32,
            height: u32,
            resizable: bool
        ) -> Self {
            let mut attrs = WindowAttributes::default();
            attrs.title = title.to_string();
            attrs.inner_size = Some(LogicalSize::new(width, height).into());
            attrs.resizable = resizable;


            Self {
                window: Arc::new(Mutex::new(None)),
                window_attr: attrs,
                initalized_window: false
            }
        }

        pub fn create_event_loop() -> EventLoop<()> {
            let event_loop = EventLoop::new().unwrap();

            event_loop.set_control_flow(ControlFlow::Poll);

            event_loop
        }

        pub fn run_engine_window(
            event_loop: EventLoop<()>,
            title: &str,
            width: u32,
            height: u32,
            resizable: bool
        ) {
            let mut app = AppWindow::new(
                title,
                width,
                height,
                resizable
            );
            event_loop.run_app(&mut app).unwrap();
        }

        pub fn is_initalized(&self) -> bool {
            self.initalized_window
        }

        pub async fn init_optional_field(&mut self, value: Window) -> Result<Arc<Mutex<Option<Window>>>, Error> {
            self.window = Arc::new(Mutex::new(Some(value)));
            Ok(self.window.clone())
        }
    }

    impl ApplicationHandler for AppWindow {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            block_on(async {
                self.window = self.init_optional_field(event_loop
                    .create_window(self.window_attr.clone())
                    .unwrap()).await.unwrap();
            });
            
            self.initalized_window = true;
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
                    self.window.lock().as_ref().unwrap().as_ref().map(|window| window.request_redraw());
                }
                _ => (),
            }
        }
    }
}
