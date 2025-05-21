// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_return_pfns {
    
    use libloading::Library;
    
    
    use vk_sys::{DevicePointers, EntryPoints};
    pub unsafe fn return_entry_points(lib: &Library) -> EntryPoints { unsafe {
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
    }}
    /*
    pub unsafe fn return_instance_pointers(lib: &Library) -> InstancePointers {
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
                    .unwrap_or(vk),
                GetPhysicalDeviceXlibPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0")
                    .unwrap_or(vk),
                CreateXcbSurfaceKHR: *lib
                    .get(b"vkCreateXcbSurfaceKHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceXcbPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0")
                    .unwrap_or(vk),
                CreateWaylandSurfaceKHR: *lib
                    .get(b"vkCreateWaylandSurfaceKHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceWaylandPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0")
                    .unwrap_or(vk),
                CreateAndroidSurfaceKHR: *lib
                    .get(b"vkCreateAndroidSurfaceKHR\0")
                    .unwrap_or(vk),
                CreateWin32SurfaceKHR: *lib
                    .get(b"vkCreateWin32SurfaceKHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceWin32PresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceDisplayPropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPropertiesKHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceDisplayPlanePropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0")
                    .unwrap_or(vk),
                GetDisplayPlaneSupportedDisplaysKHR: *lib
                    .get(b"vkGetDisplaySupportedDisplaysKHR\0")
                    .unwrap_or(vk),
                GetDisplayModePropertiesKHR: *lib
                    .get(b"vkGetDisplayModePropertiesKHR\0")
                    .unwrap_or(vk),
                CreateDisplayModeKHR: *lib
                    .get(b"vkCreateDisplayModeKHR\0")
                    .unwrap_or(vk),
                GetDisplayPlaneCapabilitiesKHR: *lib
                    .get(b"vkGetDisplayPlaneCapabilitiesKHR\0")
                    .unwrap_or(vk),
                CreateDisplayPlaneSurfaceKHR: *lib
                    .get(b"vkCreateDisplayPlaneSurfaceKHR\0")
                    .unwrap_or(vk),
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
                    .unwrap_or(vk),
                DestroyDebugUtilsMessengerEXT: *lib
                    .get(b"vkDestroyDebugUtilsMessengerEXT\0")
                    .unwrap_or(vk),
                CreateIOSSurfaceMVK: *lib
                    .get(b"vkCreateIOSSurfaceMVK\0")
                    .unwrap_or(vk),
                CreateMacOSSurfaceMVK: *lib
                    .get(b"vkCreateMacOSSurfaceMVK\0")
                    .unwrap_or(vk),
                ActivateMoltenVKLicenseMVK: *lib
                    .get(b"vkActivateMoltenVKLicenseMVK\0")
                    .unwrap_or(vk),
                ActivateMoltenVKLicensesMVK: *lib
                    .get(b"vkActivateMoltenVKLicensesMVK\0")
                    .unwrap_or(vk),
                GetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkGetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(vk),
                SetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkSetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceMetalFeaturesMVK: *lib
                    .get(b"vkGetPhysicalDeviceMetalFeaturesMVK\0")
                    .unwrap_or(vk),
                GetSwapchainPerformanceMVK: *lib
                    .get(b"vkGetSwapchainPerformanceMVK\0")
                    .unwrap_or(vk),
                CreateViSurfaceNN: *lib
                    .get(b"vkCreateViSurfaceNN\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceFeatures2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFeatures2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceProperties2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFormatProperties2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceImageFormatProperties2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceQueueFamilyProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceQueueFamilyFormatProperties2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceMemoryProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceMemoryProperties2KHR\0")
                    .unwrap_or(vk),
                GetPhysicalDeviceSparseImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0")
                    .unwrap_or(vk),
            };
        }
    }
    */

    pub unsafe fn return_device_pointers(lib: &Library) -> DevicePointers {
        todo!()
    }
}
