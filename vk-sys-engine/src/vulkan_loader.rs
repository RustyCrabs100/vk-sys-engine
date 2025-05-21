// should not need to modify any code
// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vulkan_loader {
    use libloading;
    use std::error::Error;

    pub unsafe fn load_vulkan() -> Result<libloading::Library, Box<dyn Error>> {
        let vulkan_loader: Result<libloading::Library, Box<dyn Error>> =
            Ok(libloading::Library::new("vulkan-1.dll")?);

        return vulkan_loader;
    }

    pub unsafe fn return_vulkan_item<'a, T>(
        lib: &'a libloading::Library,
        name: &'a [u8],
    ) -> Result<libloading::Symbol<'a, T>, Box<dyn Error>> {
        let symbol: libloading::Symbol<'a, T> = lib.get(name)?;

        return Ok(symbol);
    }

    pub fn close_vulkan(lib: libloading::Library) -> Result<(), Box<dyn Error>> {
        let _ = lib.close();

        Ok(())
    }
}
