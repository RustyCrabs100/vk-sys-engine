pub mod mod_create_surface {
    use async_winit::window::Window;
    use async_winit::ThreadSafety;
    use raw_window_handle::HasRawWindowHandle;
    use core::ptr::{null};
    use vk_sys::{
        AllocationCallbacks, Instance, InstancePointers, NULL_HANDLE,
        STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, SUCCESS, SurfaceKHR,
        Win32SurfaceCreateInfoKHR,
    };

    pub fn create_surface<TS: ThreadSafety>(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
        window: &Window<TS>,
        p_allocator: *const AllocationCallbacks,
    ) -> SurfaceKHR 
    where for<'a> &'a Window<TS>: HasRawWindowHandle{
        use raw_window_handle::{RawWindowHandle};

        let mut surface: SurfaceKHR = NULL_HANDLE;
        let p_surface = &mut surface as *mut SurfaceKHR;

        let window_handle = window.raw_window_handle();

        // Extract hwnd and hinstance safely
        let (hwnd, hinstance) = match window_handle {
            RawWindowHandle::UiKit(ui_kit_window_handle) => todo!(),
            RawWindowHandle::AppKit(app_kit_window_handle) => todo!(),
            RawWindowHandle::Orbital(orbital_window_handle) => todo!(),
            RawWindowHandle::Xlib(xlib_window_handle) => todo!(),
            RawWindowHandle::Xcb(xcb_window_handle) => todo!(),
            RawWindowHandle::Wayland(wayland_window_handle) => todo!(),
            RawWindowHandle::Drm(drm_window_handle) => todo!(),
            RawWindowHandle::Gbm(gbm_window_handle) => todo!(),
            RawWindowHandle::Win32(win32_window_handle) => (win32_window_handle.hwnd, win32_window_handle.hinstance),
            RawWindowHandle::WinRt(win_rt_window_handle) => todo!(),
            RawWindowHandle::Web(web_window_handle) => todo!(),
            RawWindowHandle::AndroidNdk(android_ndk_window_handle) => todo!(),
            RawWindowHandle::Haiku(haiku_window_handle) => todo!(),
            _ => panic!("Unable to find a RawWindowHandle for your OS!")
        };

        if hwnd.is_null() || hinstance.is_null() {
            panic!("Invalid Handles");
        }

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

}
