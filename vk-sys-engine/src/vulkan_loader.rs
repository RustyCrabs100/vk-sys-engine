// should not need to modify any code
// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vulkan_loader {
    use libloading;
    use std::error::Error;

    /// Loads Vulkan in (Supports Multiple Platforms)
    pub unsafe fn load_vulkan() -> Result<libloading::Library, Box<dyn Error>> {
        unsafe {
            if cfg!(target_os = "windows") {
                let vulkan_loader = Ok(libloading::Library::new("vulkan-1.dll")?);
                return vulkan_loader;
            } else if cfg!(target_os = "linux") {
                let vulkan_loader = Ok(libloading::Library::new("libvulkan.so.1")?);
                return vulkan_loader;
            } else if cfg!(target_os = "macos") {
                let vulkan_loader = Ok(libloading::Library::new("libvulkan.dylib")?);
                return vulkan_loader;
            } else {
                panic!("Operating System not Supported!");
            }
        }
    }

    /// Returns a Vulkan Item (Such as Function Pointers)
    pub unsafe fn return_vulkan_item<'a, T>(
        lib: &'a libloading::Library,
        name: &'a [u8],
    ) -> Result<libloading::Symbol<'a, T>, Box<dyn Error>> {
        unsafe {
            let symbol: libloading::Symbol<'a, T> = lib.get(name)?;

            return Ok(symbol);
        }
    }

    /// Closes Vulkan
    pub fn close_vulkan(lib: libloading::Library) -> Result<(), Box<dyn Error>> {
        let _ = lib.close();

        Ok(())
    }
}
