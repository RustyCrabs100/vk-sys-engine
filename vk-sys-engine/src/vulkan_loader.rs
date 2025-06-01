// should not need to modify any code
// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vulkan_loader {
    use core::ffi::{CStr, c_char, c_void};
    use libloading;
    use libloading::{Library, Symbol};
    use std::boxed::Box;
    use std::error::Error;

    /// Loads Vulkan in (Supports Multiple Platforms)
    pub unsafe fn load_vulkan() -> Result<libloading::Library, Box<dyn Error>> {
        unsafe {
            // Windows Vulkan Loader
            let vulkan_lib = Library::new("vulkan-1.dll")
                // Linux Vulkan Loader
                .or_else(|_| Library::new("libvulkan.so.1"))
                // MacOS Vulkan Loader
                .or_else(|_| Library::new("libvulkan.dylib"));
            return Ok(vulkan_lib?);
        }
    }

    /// Returns a usable function to load in Vulkan Function Pointers
    pub unsafe fn return_instance_function_loader(
        lib: &Library,
        instance: Option<&vk_sys::Instance>,
    ) -> impl FnMut(&CStr) -> *const c_void {
        // Loads in vkGetInstanceProcAddr
        let get_instance_proc_addr: extern "system" fn(
            vk_sys::Instance,
            *const c_char,
        ) -> vk_sys::PFN_vkVoidFunction = unsafe {
            *lib.get(b"vkGetInstanceProcAddr\0")
                .expect("Could not load Instance Function Pointer Getter (vkGetInstanceProcAddr)")
        };

        let instance_unop: &usize = instance.unwrap_or(&(0 as usize));

        let instance_owned: usize = *instance_unop;

        // Returns a closure capturing get_instance_proc_addr
        move |name: &CStr| unsafe {
            get_instance_proc_addr(instance_owned, name.as_ptr()) as *const c_void
        }
    }

    /// Closes Vulkan
    pub fn close_vulkan(lib: libloading::Library) -> Result<(), Box<dyn Error>> {
        let _ = lib.close();

        Ok(())
    }
}
