// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use async_winit::{
        event_loop::{EventLoop, EventLoopWindowTarget}, window::{Window, WindowAttributes}, ThreadSafety, ThreadUnsafe
    };
    use smol::{
        future::{FutureExt},
    };
    use std::fmt;

    /// Handles Keyboard and Mouse Inputs (Currently Unavailable)
    pub mod input_handler {
        /// Handles Keyboard Input (Stub Function)
        pub fn keyboard_input_handler() {}

        /// Handles Mouse Input (Stub Function)
        pub fn mouse_input_handler() {}
    }

    #[derive(Clone)]
    pub(crate) struct EngineWindow {
        pub(crate) window_attrs: WindowAttributes,
        pub(crate) window: Option<Window<ThreadUnsafe>>,
    }

    impl EngineWindow {
        pub async fn new() -> Self {
            Self {
                window_attrs: WindowAttributes::default(),
                window: None,
            }
        }

        pub async fn run_engine_window(&'static mut self)  {
            println!("Running!");
            let evl: EventLoop<ThreadUnsafe> = smol::block_on(Self::create_event_loop());
            let window_target: EventLoopWindowTarget = evl.window_target().clone();

            evl.block_on(async move {
                loop {
                    window_target.resumed().await;

                    let window_proper = match &self.window {
                        Some(x) => x,
                        None => &Self::create_window().await,
                    };
                    let _ = smol::block_on(Self::event_handler(&window_target, window_proper));
                }
            });
        }

        async fn create_event_loop() -> EventLoop<ThreadUnsafe> {
            EventLoop::new()
        }

        async fn event_handler<'a>(
            window_target: &EventLoopWindowTarget,
            window: &'a Window<ThreadUnsafe>,
        ) -> &'a Window<ThreadUnsafe> {
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
            println!("Debugging is fun.... (sarcasim)");
            let window: Window<ThreadUnsafe> = Window::<ThreadUnsafe>::new().await.expect("??");
            println!("Ok... Odd...");
            window
        }
    }

    impl fmt::Debug for EngineWindow {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("EngineWindow")
                .field("window", &self.window.as_ref().map(|_| "Window<ThreadUnsafe>"))
                .field("window_attrs", &"WindowAttributes")
                .finish()
        }
    }
}
