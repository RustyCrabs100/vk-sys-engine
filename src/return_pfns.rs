// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_return_pfns {
    use crate::vulkan_loader::mod_vulkan_loader::{
        return_device_function_loader, return_instance_function_loader,
    };
    // use crate::vulkan_loader::mod_vulkan_loader::return_get_instance_proc_addr_pfn;

    use libloading::Library;
    use vk_sys::{Device, DevicePointers, EntryPoints, InstancePointers};

    /// Returns EntryPoints for Instance Initalization
    pub unsafe fn return_entry_points(lib: &Library) -> EntryPoints {
        unsafe {
            EntryPoints {
                CreateInstance: *lib
                    .get(b"vkCreateInstance\0")
                    .expect("Failed to load vkCreateInstance"),
                EnumerateInstanceExtensionProperties: *lib
                    .get(b"vkEnumerateInstanceExtensionProperties\0")
                    .expect("Failed to load vkEnumerateInstanceExtensionProperties"),
                EnumerateInstanceLayerProperties: *lib
                    .get(b"vkEnumerateInstanceLayerProperties\0")
                    .expect("Failed to load vkEnumerateInstanceLayerProperties"),
            }
        }
    }

    /// Returns InstancePointers for Debugging, Physical Device Initalization,
    /// Swapchain Intialization, Image Formatting, Queue's, Surface Creations, etc
    pub unsafe fn return_instance_pointers(
        lib: &Library,
        instance: Option<&vk_sys::Instance>,
    ) -> InstancePointers {
        unsafe {
            let loader = return_instance_function_loader(lib, instance);

            InstancePointers::load(loader)
        }
    }

    /// Returns DevicePointers (Currenty a Stub Implementation)
    pub unsafe fn return_device_pointers(
        instance_ptrs: &InstancePointers,
        device: &Device,
    ) -> DevicePointers {
        let loader = unsafe { return_device_function_loader(instance_ptrs, *device) };

        DevicePointers::load(loader)
    }
}
