// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use async_winit::{
        ThreadUnsafe,
        event_loop::{EventLoop, EventLoopWindowTarget},
        window::{Window, WindowAttributes},
    };
    use smol::{
        future::{FutureExt},
    };

    /// Handles Keyboard and Mouse Inputs (Currently Unavailable)
    pub mod input_handler {
        /// Handles Keyboard Input (Stub Function)
        pub fn keyboard_input_handler() {}

        /// Handles Mouse Input (Stub Function)
        pub fn mouse_input_handler() {}
    }

    pub(crate) struct EngineWindow {
        pub(crate) window_attrs: WindowAttributes,
        pub(crate) window: Window<ThreadUnsafe>,
    }

    impl EngineWindow {
        pub async fn new() -> Self {
            Self {
                window_attrs: WindowAttributes::default(),
                window: Self::create_window().await,
            }
        }

        pub async fn run_engine_window(&'static mut self) {
            let evl: EventLoop<ThreadUnsafe> = Self::create_event_loop().await;
            let window_target: EventLoopWindowTarget = evl.window_target().clone();

            evl.block_on(async move {
                loop {
                    window_target.resumed().await;

                    let window: Window<ThreadUnsafe> = self.window.to_owned();

                    let handle_event = Self::event_handler(&window_target, window).await;

                    self.window = handle_event;
                }
            });
        }

        async fn create_event_loop() -> EventLoop<ThreadUnsafe> {
            EventLoop::new()
        }

        async fn event_handler(
            window_target: &EventLoopWindowTarget,
            window: Window<ThreadUnsafe>,
        ) -> Window<ThreadUnsafe> {
            let close = async {
                window.close_requested().wait().await;
                println!("Closing");
                true
            };

            let suspend = async {
                window_target.suspended().wait().await;
                false
            };

            let request_redraw = async {
                window.redraw_requested();
                false
            };

            let needs_exit = request_redraw.or(close).or(suspend).await;

            if needs_exit {
                window_target.exit().await;
            }

            window
        }

        async fn create_window() -> Window<ThreadUnsafe> {
            let window: Window<ThreadUnsafe> = Window::<ThreadUnsafe>::new().await.unwrap();
            window
        }
    }
}
