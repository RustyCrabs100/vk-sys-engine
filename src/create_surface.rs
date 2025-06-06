pub mod mod_create_surface {
    use vk_sys::{
        SurfaceKHR, NULL_HANDLE, Win32SurfaceCreateInfoKHR,
        STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, 
        InstancePointers, Instance, AllocationCallbacks
    };
    use winit::{window::Window};
    use core::ptr::{null};
    #[cfg(target_os = "windows")]
    pub fn create_win32_surface(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
        window: &Window,
        p_allocator: *const AllocationCallbacks
    ) -> SurfaceKHR {
        let mut surface: SurfaceKHR = NULL_HANDLE;
        let p_surface = &mut surface as *mut SurfaceKHR;

        let win32_surface_khr_create_info: Win32SurfaceCreateInfoKHR = 
            Win32SurfaceCreateInfoKHR {
                sType: STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: null(), 
                flags: 0,
                hinstance: window.raw_window_handle(),
                hwnd: window.raw_display_handle(),
            };
        let p_win32_surface_khr_create_info = &win32_surface_khr_create_info as *const Win32SurfaceCreateInfoKHR;

        let win32_surface_creation = unsafe {
            InstancePointers::CreateWin32SurfaceKHR(
                instance_ptrs,
                *instance,
                p_win32_surface_khr_create_info,
                p_allocator,
                p_surface,
            )
        };    

        if win32_surface_creation == SUCCESS {
            return surface;
        } else {
            panic!("Surface Failed to be Created");
        }
    }

    // TODO: Add X11, Xcb, and Wayland support for surfaces.
}