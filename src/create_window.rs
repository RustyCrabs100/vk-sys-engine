// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use smol::{future::block_on, Executor, LocalExecutor, Task};
    use std::{
        io::Error,
        sync::{Arc, Condvar, Mutex},
    };
    use winit::{
        dpi::{LogicalPosition, LogicalSize, Position, Size},
        window::{Window, WindowAttributes},
    };
    use winit_modular::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        future::FutEventLoop,
    };

    /// Handles Keyboard and Mouse Inputs (Currently Unavailable)
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
        pub initialized_window: Arc<(Mutex<bool>, Condvar)>,
        pub window_count: u32,
    }

    impl AppWindow {
        pub fn new(title: &str, width: u32, height: u32, resizable: bool) -> Self {
            let mut attrs = WindowAttributes::default();
            attrs.title = title.to_string();
            attrs.inner_size = Some(LogicalSize::new(width, height).into());
            attrs.resizable = resizable;

            Self {
                window: Arc::new(Mutex::new(None)),
                window_attr: attrs,
                initialized_window: Arc::new((Mutex::new(false), Condvar::new())),
                window_count: 0,
            }
        }

        pub async fn create_event_loop() -> FutEventLoop {
            EventLoop::new()
        }

        // TODO: Add back in the Fullscreen Option
        pub async fn create_window<'a>(
            &'static mut self,
            event_loop: &EventLoop,
        ) -> Result<Arc<Mutex<Option<Window>>>, Error> {
            let window = Arc::new(Mutex::new(Some(
                event_loop
                    .create_window(|builder| {
                        builder
                            .with_title(&self.window_attr.title)
                            .with_inner_size(
                                self.window_attr
                                    .inner_size
                                    .unwrap_or(Size::new(LogicalSize::new(800, 600))),
                            )
                            .with_resizable(self.window_attr.resizable)
                            .with_transparent(self.window_attr.transparent)
                            .with_decorations(self.window_attr.decorations)
                            .with_visible(self.window_attr.visible)
                            .with_min_inner_size(
                                self.window_attr
                                    .min_inner_size
                                    .unwrap_or(Size::new(LogicalSize::new(800, 600))),
                            )
                            .with_max_inner_size(
                                self.window_attr
                                    .max_inner_size
                                    .unwrap_or(Size::new(LogicalSize::new(800, 600))),
                            )
                            .with_position(
                                self.window_attr
                                    .position
                                    .unwrap_or(Position::new(LogicalPosition::new(800, 600))),
                            )
                    })
                    .await
                    .unwrap(),
            )));

            self.window_count += 1;
            Ok(window)
        }

        pub async fn run_engine_window<'a>(
            executor: &'static Executor<'a>,
            title: &'static str,
            width: u32,
            height: u32,
            resizable: bool,
        ) -> Result<(), Error> {
            winit_modular::run(move || {
                block_on(Self::runner(executor, title, width, height, resizable));
            });
        }

        async fn runner<'a>(
            executor: &'static Executor<'a>,
            title: &str,
            width: u32,
            height: u32,
            resizable: bool,
        ) {
            let mut app = AppWindow::new(title, width, height, resizable);
            let event_loop = AppWindow::create_event_loop().await.await;
            app.window = app.create_window(&event_loop).await.unwrap();

            let window_clone = app.window.clone();
            executor
                .spawn(async move {
                    event_loop
                        .run_async(move |event, control_flow, window_target| {
                            *control_flow = ControlFlow::Poll;
                            match event {
                                Event::WindowEvent {
                                    event: WindowEvent::CloseRequested,
                                    ..
                                } => {
                                    println!("Close requested");
                                    *control_flow = ControlFlow::ExitApp;
                                }
                                Event::LoopDestroyed => {
                                    println!("Window Closed");
                                }
                                _ => (),
                            }
                        })
                        .await;
                })
                .detach();
        }
    }
}
