// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_return_pfns {

    /// Returns a Dummy Function Pointer for Vulkan
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
    }

    use libloading::Library;
    use vk_sys::{DevicePointers, EntryPoints};

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
                    .map(|s|*s)
                    .unwrap_or(vk_dummy_pfn_creator!(CreateXlibSurfaceKHR, (a: vk_sys::Instance, b: *const vk_sys::XlibSurfaceCreateInfoKHR, c: *const AllocationCallbacks, d: *mut vk_sys::SurfaceKHR), Result, 0)),
                GetPhysicalDeviceXlibPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!(GetPhysicalDeviceXcbPresentationSupportKHR, (a: vk_sys::PhysicalDevice, b: u32, c: *const AllocationCallbacks, d: *mut vk_sys::SurfaceKHR), Bool32, 0)),
                CreateXcbSurfaceKHR: *lib
                    .get(b"vkCreateXcbSurfaceKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!(CreateXcbSurfaceKHR, (a: vk_sys::Instance, b: *const vk_sys::XcbSurfaceCreateInfoKHR, c: *const AllocationCallbacks, d: *mut vk_sys::SurfaceKHR), Result, 0)),
                GetPhysicalDeviceXcbPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!(GetPhysicalDeviceXcbPresentationSupportKHR, (a: vk_sys::PhysicalDevice, b: u32, c: *mut c_void, d: u32), Bool32, 0)),
                CreateWaylandSurfaceKHR: *lib
                    .get(b"vkCreateWaylandSurfaceKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!(CreateWaylandSurfaceKHR, (a: vk_sys::Instance, b: *const vk_sys::WaylandSurfaceCreateInfoKHR, c: *const AllocationCallbacks, d: *mut vk_sys::SurfaceKHR), Bool32, 0)),
                GetPhysicalDeviceWaylandPresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!(GetPhysicalDeviceWaylandPresentationSupportKHR, (a: vk_sys::PhysicalDevice, b: u32, c: *mut c_void), Bool32, 0)),
                CreateAndroidSurfaceKHR: *lib
                    .get(b"vkCreateAndroidSurfaceKHR\0")
                    .map(|s| *s)
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateWin32SurfaceKHR: *lib
                    .get(b"vkCreateWin32SurfaceKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceWin32PresentationSupportKHR: *lib
                    .get(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceDisplayPropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPropertiesKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceDisplayPlanePropertiesKHR: *lib
                    .get(b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetDisplayPlaneSupportedDisplaysKHR: *lib
                    .get(b"vkGetDisplaySupportedDisplaysKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetDisplayModePropertiesKHR: *lib
                    .get(b"vkGetDisplayModePropertiesKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateDisplayModeKHR: *lib
                    .get(b"vkCreateDisplayModeKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetDisplayPlaneCapabilitiesKHR: *lib
                    .get(b"vkGetDisplayPlaneCapabilitiesKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateDisplayPlaneSurfaceKHR: *lib
                    .get(b"vkCreateDisplayPlaneSurfaceKHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
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
                    .unwrap_or(vk_dummy_pfn_creator!()),
                DestroyDebugUtilsMessengerEXT: *lib
                    .get(b"vkDestroyDebugUtilsMessengerEXT\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateIOSSurfaceMVK: *lib
                    .get(b"vkCreateIOSSurfaceMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateMacOSSurfaceMVK: *lib
                    .get(b"vkCreateMacOSSurfaceMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                ActivateMoltenVKLicenseMVK: *lib
                    .get(b"vkActivateMoltenVKLicenseMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                ActivateMoltenVKLicensesMVK: *lib
                    .get(b"vkActivateMoltenVKLicensesMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkGetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                SetMoltenVKDeviceConfigurationMVK: *lib
                    .get(b"vkSetMoltenVKDeviceConfigurationMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceMetalFeaturesMVK: *lib
                    .get(b"vkGetPhysicalDeviceMetalFeaturesMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetSwapchainPerformanceMVK: *lib
                    .get(b"vkGetSwapchainPerformanceMVK\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                CreateViSurfaceNN: *lib
                    .get(b"vkCreateViSurfaceNN\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceFeatures2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFeatures2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceFormatProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceImageFormatProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceQueueFamilyProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceQueueFamilyFormatProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceMemoryProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceMemoryProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
                GetPhysicalDeviceSparseImageFormatProperties2KHR: *lib
                    .get(b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0")
                    .unwrap_or(vk_dummy_pfn_creator!()),
            };
        }
    }
    */
    
    /// Returns DevicePointers (Currenty a Stub Implementation)
    pub unsafe fn return_device_pointers(lib: &Library) -> DevicePointers {
        todo!()
    }
}
