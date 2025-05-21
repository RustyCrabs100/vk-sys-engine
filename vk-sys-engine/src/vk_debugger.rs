// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vk_debugger {
    use core::ptr::{copy_nonoverlapping, null_mut};
    use std::alloc::{Layout, alloc, dealloc};
    use std::os::raw::c_void;
    use vk_sys::SystemAllocationScope;

    pub const fn return_validation() -> bool {
        if cfg!(debug_assertions) {
            return true;
        }
        return false;
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
}
