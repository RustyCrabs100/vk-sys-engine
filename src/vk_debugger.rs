// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

pub mod mod_vk_debugger {
    // 16 GB is the maximum resonable size vulkan can allocate
    const MAX_REASONABLE_SIZE: usize = 16 * 1024 * 1024 * 1024;

    use crate::static_c_char_array;
    use core::ffi::{c_char, c_void};
    use core::ptr::{copy_nonoverlapping, null, null_mut};
    use std::alloc::{Layout, alloc, dealloc};
    use std::cmp;
    use vk_sys::{
        AllocationCallbacks, DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
        DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT, DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
        DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT, DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT, DebugUtilsMessageSeverityFlagBitsEXT,
        DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT,
        DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, FALSE, InstancePointers,
        InternalAllocationType, LayerProperties,
        STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT, SUCCESS, SystemAllocationScope,
    };

    /// Returns Validation Support (For times when you can't immidiately check Layers)
    pub const fn return_validation() -> bool {
        if cfg!(debug_assertions) {
            return true;
        }
        false
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
    ) -> vk_sys::Bool32 {
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
            "[ERROR]: {0:?}, Type: {error_type:?}, Severity: {error_severity:?}. ",
            unsafe { (*error_info).pMessage }
        );

        // Returns VK_FALSE after ending.
        FALSE
    }

    /// Returns the Debug Messenger Info
    pub extern "system" fn vk_debug_messenger_init() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT {
            sType: STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: null(),
            flags: 0,
            // Only warn if the callback is a warning or an error
            messageSeverity: DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
                | DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
            // Only warn if the callback is for Validation or to Inform of a more optimized method
            messageType: DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
                | DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
            // Informs vulkan of the callback function
            pfnUserCallback: vk_debug_callback,
            pUserData: null_mut(),
        }
    }

    /// Stub Implementation
    pub fn return_debug_messenger(
        instance_ptrs: &vk_sys::InstancePointers,
        instance: &vk_sys::Instance,
        p_debug_create_info: *const DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_messenger: *const DebugUtilsMessengerEXT,
        validation: bool,
    ) -> vk_sys::Result {
        if !validation {
            return vk_sys::ERROR_EXTENSION_NOT_PRESENT;
        }

        let result: vk_sys::Result = unsafe {
            InstancePointers::CreateDebugUtilsMessengerEXT(
                instance_ptrs,
                *instance,
                p_debug_create_info,
                p_allocator,
                p_messenger,
            )
        };

        if result == SUCCESS {
            return result;
        }
        vk_sys::ERROR_EXTENSION_NOT_PRESENT
    }

    pub fn destroy_debug_messenger(
        instance_ptrs: &vk_sys::InstancePointers,
        instance: &vk_sys::Instance,
        debug_messenger: vk_sys::DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks,
        validation: bool,
    ) {
        if !validation {
            return;
        }

        let result: vk_sys::Result = unsafe {
            InstancePointers::DestroyDebugUtilsMessengerEXT(
                instance_ptrs,
                *instance,
                debug_messenger,
                p_allocator,
            )
        };

        if result == SUCCESS {
        } else {
            panic!("Failed to delete debug messenger!");
        }
    }

    /// Returns AllocationCallbacks.
    pub fn return_allocation_callbacks() -> AllocationCallbacks {
        AllocationCallbacks {
            pUserData: null_mut(),
            pfnAllocation: allocation_fn,
            pfnReallocation: reallocation_fn,
            pfnFree: free_fn,
            pfnInternalAllocation: internal_alloc_notify,
            pfnInternalFree: internal_free_notify,
        }
    }

    /// This is for FFI & Rust Safe Deallocation
    /// (As Vulkan does not allow you to provide size and alignment
    ///  to their free function pointer as an input.)
    #[repr(C)]
    struct AllocationInfo {
        size: usize,
        alignment: usize,
    }

    /// Provides AllocationCallbacks' Allocation Field
    extern "system" fn allocation_fn(
        _p_user_data: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: SystemAllocationScope,
    ) -> *mut c_void {
        // Adds checking for valid memory allocation
        if size == 0 || alignment == 0 || !alignment.is_power_of_two() || size > MAX_REASONABLE_SIZE
        {
            eprintln!(
                "Vulkan attempted an Allocation of size {} and alignment of size {}, which is invalid",
                size, alignment
            );
            return null_mut();
        }

        // println!("Allocating {} bytes with alignment {}", size, alignment);

        // Creates new Metadata
        let header_layout = Layout::new::<AllocationInfo>();
        // Creates actual Layout for Vulkan
        let mem_layout = Layout::from_size_align(size, alignment)
            .expect("Failed to Layout Memory for custom Allocation");
        // Initalizes Metadata
        let (layout, offset) = header_layout.extend(mem_layout).unwrap();

        unsafe {
            // Allocates memory
            let mem = alloc(layout);
            // Panics if no memory was allocated
            if mem.is_null() {
                panic!("Allocation Function Attempted to Allocate 0 Bytes of Memory");
            }

            // Initalizes Metadata as a pointer
            let header_mem = mem as *mut AllocationInfo;

            // Adds metadata to pointer.
            (*header_mem).size = size;
            (*header_mem).alignment = alignment;

            // Adds offset to mem
            

            // returns mem
            mem.add(offset) as *mut c_void
        }
    }

    /// Provides AllocationCallbacks' Reallocation Field
    extern "system" fn reallocation_fn(
        _p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        _scope: SystemAllocationScope,
    ) -> *mut c_void {
        // Adds checking for valid memory reallocation
        if size == 0 || alignment == 0 || !alignment.is_power_of_two() || size > MAX_REASONABLE_SIZE
        {
            eprintln!(
                "Vulkan attempted an Allocation of size {} and alignment of size {}, which is invalid",
                size, alignment
            );
            return null_mut();
        }

        // Checks if reallocation is valid
        if p_original.is_null() {
            return allocation_fn(_p_user_data, size, alignment, _scope);
        }

        // println!("Reallocating {} bytes of memory", size);

        // Creates new Layout for Metadata
        let metadata_layout = Layout::new::<AllocationInfo>();

        // Gets the offset of the Metadata
        let offset = metadata_layout.size();

        // Gets the old AllocationInfo
        let old_alloc_info = unsafe { (p_original as *mut u8).sub(offset) as *mut AllocationInfo };

        // Gets the old Allocation Size
        let old_alloc_size = unsafe { (*old_alloc_info).size };

        // Recreate Memory
        let new_mem = allocation_fn(_p_user_data, size, alignment, _scope);

        // Checks if new_mem is null, returns if empty
        if new_mem.is_null() {
            return null_mut();
        }

        // Gets minimum size
        let min_mem_size = cmp::min(size, old_alloc_size);

        unsafe {
            copy_nonoverlapping(p_original as *const u8, new_mem as *mut u8, min_mem_size);
            free_fn(_p_user_data, p_original);
        }

        new_mem
    }

    /// Provides AllocationCallbacks' Free Field
    extern "system" fn free_fn(_p_user_data: *mut c_void, p_memory: *mut c_void) {
        // println!("Freeing {:?} memory", p_memory);
        if p_memory.is_null() {
            eprintln!("Vulkan attempted to free 0 bytes of memory");
            return;
        }

        // Creates new Layout for Metadata
        let alloc_layout_uninit = Layout::new::<AllocationInfo>();

        // Creates the memory offset
        let offset = alloc_layout_uninit.size();

        unsafe {
            // Get memory metadata
            let alloc_ptr: *mut AllocationInfo =
                (p_memory as *mut u8).sub(alloc_layout_uninit.size()) as *mut AllocationInfo;

            // Gets memory size
            let alloc_size: usize = (*alloc_ptr).size;

            // Gets memory alignment
            let alloc_alignment: usize = (*alloc_ptr).alignment;

            // Prevents further crashing, as user_supplied AllocationCallbacks
            // Functions will allocate with a size & alignment of 0. This causes crashes down the line
            if alloc_size == 0
                || alloc_alignment == 0
                || !alloc_alignment.is_power_of_two()
                || alloc_size > MAX_REASONABLE_SIZE
            {
                eprintln!(
                    "Vulkan attempted to free memory of size {} and alignment of size {}, which is invalid",
                    alloc_size, alloc_alignment
                );
                return;
            }
            // Gets proper memory
            let alloc_layout =
                Layout::from_size_align(alloc_size, alloc_alignment).unwrap_or_else(|_| panic!("Invalid layout: size = {}, alignment = {}",
                    alloc_size, alloc_alignment));

            let (layout, _offset) = alloc_layout_uninit.extend(alloc_layout).unwrap();

            dealloc(alloc_ptr as *mut u8, layout);
        }
    }
    /// Notifies us if vulkan allocates memory behind the scenes
    extern "system" fn internal_alloc_notify(
        _p_user_data: *mut c_void,
        mem_size: usize,
        _internal_alloc_type: InternalAllocationType,
        system_alloc_scope: SystemAllocationScope,
    ) -> *mut c_void {
        println!(
            "Internal Allocation Notification: {} bytes are being allocated in {:?}",
            mem_size, system_alloc_scope
        );
        null_mut()
    }
    /// Notifies us if vulkan allocates memory behind the scenes
    extern "system" fn internal_free_notify(
        _p_user_data: *mut c_void,
        mem_size: usize,
        _internal_alloc_type: InternalAllocationType,
        system_alloc_scope: SystemAllocationScope,
    ) -> *mut c_void {
        println!(
            "Internal Memory Free Notification: {} bytes are being freed in {:?}",
            mem_size, system_alloc_scope
        );
        null_mut()
    }
}
