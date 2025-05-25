// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_window {
    use minifb::{Key, Window, WindowOptions};

    /// Handles Keyboard and Mouse Inputs (Currently Unavaliable)
    pub mod input_handler {

        /// Handles Keyboard Input (Stub Function)
        pub fn keyboard_input_handler() {}
        
        /// Handles Mouse Input (Stub Function)
        pub fn mouse_input_handler() {}
    }

    /// Creates a Window
    pub fn window_creation(height: usize, width: usize) {
        // Creates a buffer
        let mut buffer: Vec<u32> = vec![0; width * height];

        // Creates a Window, Panics if fails.
        let mut window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions {
                resize: false,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.set_target_fps(60);
        
        while window.is_open() && !window.is_key_down(Key::Escape) {
            for i in buffer.iter_mut() {
                *i = 0
            }

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }
}
