// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_return_pfns {
    use crate::vulkan_loader::mod_vulkan_loader::return_instance_function_loader;
    // use crate::vulkan_loader::mod_vulkan_loader::return_get_instance_proc_addr_pfn;
    use vk_sys::PFN_vkVoidFunction;

    /// Returns a Dummy Function Pointer for Vulkan
    /* Temporary Commenting for testing (do we need this macro???) 
    #[macro_export]
    macro_rules! vk_dummy_pfn_creator {
        ($name:ident, ($($arg:ident: $type_input:ty),*), $ret:ty, $ret_val:expr) => {{
            extern "system" fn $name($($arg: $type_input),*) -> $ret {
                eprintln!("Dummy function called in place for {}", stringify!($name));
                $ret_val
            }
            $name as extern "system" fn($($type_input),*) -> $ret
        }};

        ($name:ident, ($($arg:ident: $type_input:ty),*)) => {{
            extern "system" fn $name($($arg: $type_input),*) {
                eprintln!("Dummy function called in place for {}", stringify!($name));
            }
            $name as extern "system" fn($($type_input),*)
        }};
    } */

    use libloading::Library;
    use vk_sys::{DevicePointers, InstancePointers, EntryPoints};

    /// Returns EntryPoints for Instance Initalization
    pub unsafe fn return_entry_points(lib: &Library) -> EntryPoints {
        unsafe {
            return EntryPoints {
                CreateInstance: *lib
                    .get(b"vkCreateInstance\0")
                    .expect("Failed to load vkCreateInstance"),
                EnumerateInstanceExtensionProperties: *lib
                    .get(b"vkEnumerateInstanceExtensionProperties\0")
                    .expect("Failed to load vkEnumerateInstanceExtensionProperties"),
                EnumerateInstanceLayerProperties: *lib
                    .get(b"vkEnumerateInstanceLayerProperties\0")
                    .expect("Failed to load vkEnumerateInstanceLayerProperties"),
            };
        }
    }

    /// Returns InstancePointers for Debugging, Physical Device Initalization,
    /// Swapchain Intialization, Image Formatting, Queue's, Surface Creations, etc
    pub unsafe fn return_instance_pointers(lib: &Library, instance: Option<&vk_sys::Instance>) -> InstancePointers {
        let loader = return_instance_function_loader(lib, instance);

        InstancePointers::load(loader)
    }

    /// Returns DevicePointers (Currenty a Stub Implementation)
    pub unsafe fn return_device_pointers(lib: &Library) -> DevicePointers {
        todo!()
    }
}
