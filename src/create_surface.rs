pub mod mod_create_surface {
    use vk_sys::{
        SurfaceKHR, NULL_HANDLE, Win32SurfaceCreateInfoKHR,
        STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, 
        InstancePointers, Instance, AllocationCallbacks, SUCCESS
    };
    use winit::{window::Window};
    use core::ptr::{null, null_mut};
    use core::ffi::c_void;
    #[cfg(target_os = "windows")]
    pub fn create_win32_surface(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
        window: &Window,
        p_allocator: *const AllocationCallbacks
    ) -> SurfaceKHR {
        use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawWindowHandle};

        let mut surface: SurfaceKHR = NULL_HANDLE;
        let p_surface = &mut surface as *mut SurfaceKHR;

        // Was going to fix but too lazy to.
        let window_owned = window;

        let window_handle = match window_owned.window_handle() {
           Ok(handle) => handle.as_raw(),
           Err(e) => {
                eprintln!("Failed to get window handle: {:?}", e);
                return surface;
           }
        };

        let display_handle = match window_owned.display_handle() {
           Ok(handle) => handle.as_raw(),
           Err(e) => {
                eprintln!("Failed to get display handle: {:?}", e);
                return surface;
            }
        };

        // Extract hwnd and hinstance safely
        let (hwnd, hinstance) = match window_handle {
            RawWindowHandle::Win32(handle) => (handle.hwnd.get() as *mut c_void, handle.hinstance.map_or(null_mut(), |h| h.get() as *mut c_void)),
            _ => {
                eprintln!("Unsupported window handle type");
               return surface;
            }
        };

        let win32_surface_khr_create_info: Win32SurfaceCreateInfoKHR = 
            Win32SurfaceCreateInfoKHR {
                sType: STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: null(), 
                flags: 0,
                hinstance: hinstance,
                hwnd: hwnd,
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