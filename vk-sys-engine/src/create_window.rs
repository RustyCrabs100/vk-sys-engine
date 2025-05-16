pub mod mod_window {
    use minifb::{Key, Window, WindowOptions};

    pub mod input_handler {}

    pub fn window_creation(width: usize, height: usize) {
        let mut buffer: Vec<u32> = vec![0; width * height];

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
                *i = 0 // write something more funny here!
            }

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }
}
