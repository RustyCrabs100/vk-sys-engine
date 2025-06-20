// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_utils {
    use smol::block_on;

    /// Returns a readable version in u32.
    pub fn make_version(major: u8, minor: u8, patch: u16, build: u8) -> u32 {
        ((major as u32) << 24) | ((minor as u32) << 16) | ((patch as u32) << 8) | (build as u32)
    }
    /// Parses a readable version in u32 into it's individual components.
    pub async fn parse_version(version: u32) -> (u8, u8, u16, u8) {
        let mut major: u8 = 0;
        let mut minor: u8 = 0;
        let mut patch: u16 = 0;
        let mut build: u8 = 0;
        block_on(async {
            major = (version >> 24) as u8;
            minor = ((version >> 16) & 0xFF) as u8;
            patch = ((version >> 8) & 0xFF) as u16;
            build = (version & 0xFF) as u8;
        });

        (major, minor, patch, build)
    }

    /// Returns a Static c_char Array that Vulkan can read.
    #[macro_export]
    macro_rules! static_c_char_array {
        ($s:expr) => {{
            // This is done as it's already in byte form
            const BYTES: &[u8] = $s;
            let mut buffer: [c_char; 256] = [0; 256]; // Initialize with null terminators

            let length = if BYTES.len() < 255 { BYTES.len() } else { 255 }; // Ensure space for null terminator
            let mut i = 0;

            while i < length {
                buffer[i] = BYTES[i] as c_char;
                i += 1;
            }

            buffer[length] = 0; // Explicitly set the null terminator

            buffer // Return correctly formatted array
        }};
    }
}
