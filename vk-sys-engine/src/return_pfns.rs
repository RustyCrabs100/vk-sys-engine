pub mod mod_return_pfns {
    use libloading::Library;
    use std::mem::transmute;
    use std::ptr::null_mut;
    use libloading::Symbol;
    use vk_sys::{DevicePointers, EntryPoints, InstancePointers, Result};
    pub unsafe fn return_entry_points(lib: &Library) -> EntryPoints {
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

    pub unsafe fn return_instance_pointers(lib: &Library) -> InstancePointers {
        let unwrapped_dummy_fn = match dummy_function(1,1,1,1) {
            Some(a) => println!("HOWWWWWWW"),
            None => println!("Expected Value for uninitalized values")
        };

        let unwrapped_dummy_func = unwrapped_dummy_fn;
        
        unsafe {
            return InstancePointers {
                DestroyInstance: *lib
                    .get(b"vkDestroyInstance\0")
                    .expect("Failed to load vkDestroyInstance"),
                GetDeviceProcAddr: *lib
                    .get(b"vkGetDeviceProcAddr\0")
                    .expect("Failed to load vkGetDeviceProcAddr"),
                EnumeratePhysicalDevices: *lib
                    .get(b"vkEnumeratePhysicalDevices\0")
                    .expect("Failed to load vkEnumeratePhysicalDevices"),
                EnumerateDeviceExtensionProperties: *lib
                    .get(b"vkEnumerateDeviceExtensionProperties\0")
                    .expect("Failed to load vkEnumerateDeviceExtensionProperties"),
                EnumerateDeviceLayerProperties: *lib
                    .get(b"vkEnumerateDeviceLayerProperties\0")
                    .expect("Failed to load vkEnumerateDeviceLayerProperties"),
                CreateDevice: *lib
                    .get(b"vkCreateDevice\0")
                    .expect("Failed to load vkCreateDevice"),
                GetPhysicalDeviceFeatures: *lib
                    .get(b"vkGetPhysicalDeviceFeatures\0")
                    .expect("Failed to load vkGetPhysicalDeviceFeatures"),
                GetPhysicalDeviceFormatProperties: *lib
                    .get(b"vkGetPhysicalDeviceFormatProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceFormatProperties"),
                GetPhysicalDeviceImageFormatProperties: *lib
                    .get(b"vkGetPhysicalDeviceImageFormatProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceImageFormatProperties"),
                GetPhysicalDeviceProperties: *lib
                    .get(b"vkGetPhysicalDeviceProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceProperties"),
                GetPhysicalDeviceQueueFamilyProperties: *lib
                    .get(b"vkGetPhysicalDeviceQueueFamilyProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceQueueFamilyProperties"),
                GetPhysicalDeviceMemoryProperties: *lib
                    .get(b"vkGetPhysicalDeviceMemoryProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceMemoryProperties"),
                GetPhysicalDeviceSparseImageFormatProperties: *lib
                    .get(b"vkGetPhysicalDeviceSparseImageFormatProperties\0")
                    .expect("Failed to load vkGetPhysicalDeviceSparseImageFormatProperties"),
                DestroySurfaceKHR: *lib
                    .get(b"vkDestroySurfaceKHR\0")
                    .expect("Failed to load vkDestroySurfaceKHR"),
                CreateXlibSurfaceKHR: *lib
                    .get(b"vkCreateXlibSurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceXlibPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateXcbSurfaceKHR: *lib
                    .get(b"vkCreateXcbSurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceXcbPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateWaylandSurfaceKHR: *lib
                    .get(b"vkCreateWaylandSurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceWaylandPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateAndroidSurfaceKHR: *lib
                    .get(b"vkCreateAndroidSurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateWin32SurfaceKHR: *lib
                    .get(b"vkCreateWin32SurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceWin32PresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceDisplayPropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPropertiesKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceDisplayPlanePropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetDisplayPlaneSupportedDisplaysKHR: *lib
                    .get(b"vkGetDisplaySupportedDisplaysKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetDisplayModePropertiesKHR: *lib
                    .get(b"vkGetDisplayModePropertiesKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateDisplayModeKHR: *lib
                    .get(b"vkCreateDisplayModeKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetDisplayPlaneCapabilitiesKHR: *lib
                    .get(b"vkGetDisplayPlaneCapabilitiesKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateDisplayPlaneSurfaceKHR: *lib
                    .get(b"vkCreateDisplayPlaneSurfaceKHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceSurfaceSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceSurfaceSupportKHR\0")
                    .expect("Failed to load vkCreateDisplayPlaneSurfaceKHR"),
                GetPhysicalDeviceSurfaceCapabilitiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0")
                    .expect("Failed to load vkGetPhysicalDeviceSurfaceCapabilitiesKHR"),
                GetPhysicalDeviceSurfaceFormatsKHR: *lib
                    .get(b"vkGetPhysicalDeviceSurfaceFormatsKHR\0")
                    .expect("Failed to load vkGetPhysicalDeviceSurfaceFormatsKHR"),
                GetPhysicalDeviceSurfacePresentModesKHR: *lib
                    .get(b"vkGetPhysicalDeviceSurfacePresentModesKHR\0")
                    .expect("Failed to load vkGetPhysicalDeviceSurfacePresentModesKHR"),
                CreateDebugUtilsMessengerEXT: *lib
                    .get(b"vkCreateDebugUtilsMessengerEXT\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                DestroyDebugUtilsMessengerEXT: *lib
                    .get(b"vkDestroyDebugUtilsMessengerEXT\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateIOSSurfaceMVK: *lib
                    .get(b"vkCreateIOSSurfaceMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateMacOSSurfaceMVK: *lib
                    .get(b"vkCreateMacOSSurfaceMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                ActivateMoltenVKLicenseMVK: *lib
                    .get(b"vkActivateMoltenVKLicenseMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                ActivateMoltenVKLicensesMVK: *lib
                    .get(b"vkActivateMoltenVKLicensesMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkGetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                SetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkSetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceMetalFeaturesMVK: *lib
                    .get(b"vkGetPhysicalDeviceMetalFeaturesMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetSwapchainPerformanceMVK: *lib
                    .get(b"vkGetSwapchainPerformanceMVK\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                CreateViSurfaceNN: *lib
                    .get(b"vkCreateViSurfaceNN\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceFeatures2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFeatures2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFormatProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceImageFormatProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceQueueFamilyProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceQueueFamilyFormatProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceMemoryProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceMemoryProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
                GetPhysicalDeviceSparseImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0")
                    .unwrap_or(transmute(unwrapped_dummy_func)),
            };
        }
    }

    extern "system" fn dummy_function<'a, T>(_: T, _: T, _: T, _: T) -> Option<Symbol<'a, T>> {
        eprintln!("This function doesn't work if your trying to call it.");
        eprintln!("Note: This is from the developer behind this Game Engine");
        //let symbol: Option<Symbol<_>> = None;
        //return symbol
        unimplemented!();
    }

    pub unsafe fn return_device_pointers(lib: &Library) -> DevicePointers {
        todo!()
    }
}
