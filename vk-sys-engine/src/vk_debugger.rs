// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vk_debugger {
    use crate::static_c_char_array;
    use core::ffi::{c_char, c_void};
    use core::ptr::copy_nonoverlapping;
    use std::alloc::{Layout, alloc};
    use vk_sys::{
        DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT, DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT,
        DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessageSeverityFlagBitsEXT,
        DebugUtilsMessengerCallbackDataEXT, LayerProperties, SystemAllocationScope,
        FALSE
    };

    /// Returns Validation Support (For times when you can't immidiately check Layers)
    pub const fn return_validation() -> bool {
        if cfg!(debug_assertions) {
            return true;
        }
        return false;
    }

    /// Same thing as return_validation(), but checks Vulkan Instance Layers
    pub fn checking_validation_support(layers: &[LayerProperties]) -> bool {
        if cfg!(debug_assertions) {
            for layer in layers {
                if layer.layerName == static_c_char_array!(b"VK_LUNARG_KHRONOS_validation\0") {
                    return true;
                }
            }
            return false;
        }
        false
    }


    /// Provides a Debug Callback for Vulkan
    pub extern "system" fn vk_debug_callback(
        error_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        error_type: DebugUtilsMessageTypeFlagsEXT,
        error_info: *const DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut c_void,
    ) -> u32 {

        // Exits out if error_severity is for verbose callbacks
        if error_severity == DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT {
            return 3_294_956_295u32;
        }

        // Exits out if error_type is for non-validation or performance calls
        if error_type == DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT {
            return 3_294_956_295u32;
        }

        // else, Prints out the error
        println!(
            "Error: {0:?}, Type: {error_type:?}, Severity: {error_severity:?}. ",
            unsafe{(*error_info).pMessage}
        );

        // Returns VK_FALSE after ending.
        FALSE
    }


    /// Stub Implementation
    pub fn return_debug_messenger() {

    }

    /*
    pub fn return_allocation_callbacks(return_allocation: bool) -> Option<AllocationCallbacks> {
        if return_allocation {
            return AllocationCallbacks {
                pUserData: null_mut(),
            }
        }  else {
            return None
        }
    }
    */

    /// Provides AllocationCallbacks' Allocation Field
    extern "system" fn allocation_fn(
        _p_user_data: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: SystemAllocationScope,
    ) -> *mut c_void {
        println!("Allocating {} bytes of memory", size);

        let mem_layout = Layout::from_size_align(size, alignment)
            .expect("Failed to Layout Memory for custom Allocation");

        unsafe { alloc(mem_layout) as *mut c_void }
    }

    /// Provides AllocationCallbacks' Reallocation Field
    extern "system" fn reallocation_fn(
        _p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: SystemAllocationScope,
    ) -> *mut c_void {
        println!("Reallocating {} bytes of memory", size);

        let mem_layout = Layout::from_size_align(size, alignment)
            .expect("Failed to Layout Reallocated Memory for custom Allocation");

        let new_mem = unsafe { alloc(mem_layout) as *mut c_void };

        unsafe { copy_nonoverlapping(p_original, new_mem, size) };

        return new_mem;
    }

    // Still waiting for Implementations for Free calls, and for Internal Notifications
}
