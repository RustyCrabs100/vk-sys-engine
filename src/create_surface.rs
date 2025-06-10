pub mod mod_create_surface {
    use core::ffi::c_void;
    use core::ptr::{null, null_mut};
    use vk_sys::{
        AllocationCallbacks, Instance, InstancePointers, NULL_HANDLE,
        STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, SUCCESS, SurfaceKHR,
        Win32SurfaceCreateInfoKHR,
    };
    use winit::window::Window;
    #[cfg(target_os = "windows")]
    pub fn create_win32_surface(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
        window: &Window,
        p_allocator: *const AllocationCallbacks,
    ) -> SurfaceKHR {
        use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

        let mut surface: SurfaceKHR = NULL_HANDLE;
        let p_surface = &mut surface as *mut SurfaceKHR;

        let window_handle = HasRawWindowHandle::raw_window_handle(window);

        // Extract hwnd and hinstance safely
        let (hwnd, hinstance) = match window_handle {
            RawWindowHandle::Win32(handle) => {
                (handle.hwnd as *mut c_void, handle.hinstance as *mut c_void)
            }
            _ => {
                eprintln!("Unsupported window handle type");
                return surface;
            }
        };

        let win32_surface_khr_create_info: Win32SurfaceCreateInfoKHR = Win32SurfaceCreateInfoKHR {
            sType: STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: null(),
            flags: 0,
            hinstance: hinstance,
            hwnd: hwnd,
        };
        let p_win32_surface_khr_create_info =
            &win32_surface_khr_create_info as *const Win32SurfaceCreateInfoKHR;

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
