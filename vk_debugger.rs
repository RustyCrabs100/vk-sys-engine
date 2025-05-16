pub mod mod_vk_debugger {
    use core::ptr::{null_mut, copy_nonoverlapping};
    use std::os::raw::c_void;
    use std::alloc::{Layout, alloc, dealloc};
    use vk_sys::VkSystemAllocationScope;
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
        _scope: VkSystemAllocationScope
    ) -> *mut c_void {
        println!("Allocating {} bytes of memory", size);

        let mem_layout = Layout::from_size_align(size, alignment).expect("Failed to Layout Memory for custom Allocation");

        unsafe {alloc(mem_layout) as *mut c_void}

    }

    extern "system" fn reallocation_fn(
        _p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: VkSystemAllocationScope
    ) -> *mut c_void {
        println!("Reallocating {} bytes of memory", size);

        let mem_layout =Layout::from_size_align(size, alignmen).expect("Failed to Layout Reallocated Memory for custom Allocation");

        let new_mem = unsafe{alloc(mem_layout) as *mut c_void};

        unsafe{copy_nonoverlapping(p_original, new_mem, size)};

        return new_mem
    }
}