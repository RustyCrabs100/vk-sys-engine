pub mod mod_instance_creation {
    use crate::static_c_char_array;
    use crate::vk_debugger::mod_vk_debugger::vk_debug_messenger_init;
    use core::ffi::{c_char, c_void};
    use core::mem::zeroed;
    use core::ptr::{null, null_mut};
    use std::alloc::{Layout, alloc};
    use vk_sys::{
        AllocationCallbacks, ApplicationInfo, DebugUtilsMessengerCreateInfoEXT, EntryPoints,
        ExtensionProperties, Instance, InstanceCreateInfo, LayerProperties, Result,
        STRUCTURE_TYPE_APPLICATION_INFO, STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS,
    };
    /// Creates Vulkan Instance
    pub fn create_instance(
        entry_pointers: &EntryPoints,
        application_version: u32,
        engine_version: u32,
        vulkan_version: u32,
        allocation_callbacks: &AllocationCallbacks,
        extension_count: u32,
        extensions: Vec<*const i8>,
        layer_count: u32,
        layers: Vec<*const i8>,
        validation: bool,
    ) -> Instance {
        let application_name_wrapper: [c_char; 256] = static_c_char_array!(b"Rust Game Engine\0");
        let engine_name_wrapper: [c_char; 256] = static_c_char_array!(b"Rustic\0");

        // Sets application info
        let app_info: ApplicationInfo = ApplicationInfo {
            sType: STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: application_name_wrapper.as_ptr(),
            applicationVersion: application_version,
            pEngineName: engine_name_wrapper.as_ptr(),
            engineVersion: engine_version,
            apiVersion: vulkan_version,
        };
        // wraps ApplicationInfo in a type that InstanceCreateInfo can read
        let application_info_wrapper: *const ApplicationInfo = &app_info;
        // creates Instance Info
        let mut instance_create_info: InstanceCreateInfo = InstanceCreateInfo {
            sType: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: application_info_wrapper,
            enabledLayerCount: layer_count,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extension_count,
            ppEnabledExtensionNames: extensions.as_ptr(),
        };

        let debug_create_info: DebugUtilsMessengerCreateInfoEXT = vk_debug_messenger_init();
        if validation {
            instance_create_info.pNext =
                &debug_create_info as *const DebugUtilsMessengerCreateInfoEXT as *const c_void;
        }
        // Initalizes an empty handle to the Vulkan Instance
        let mut instance: Instance = unsafe { zeroed() };
        // wraps Instance in a type that CreateInstance can use.
        let instance_pointer: *mut Instance = &mut instance;
        // Creates Instance
        let result: Result = unsafe {
            EntryPoints::CreateInstance(
                entry_pointers,
                &instance_create_info,
                allocation_callbacks as *const AllocationCallbacks,
                instance_pointer,
            )
        };
        // Checks if Instance Creation was successful
        if result == SUCCESS {
            println!("vkCreateInstance correctly Initalized");
        } else {
            panic!("vkCreateInstance failed, can't continue");
        }

        instance
    }
    /// Returns Vulkan Instance Extensions
    pub unsafe fn return_instance_extensions(
        entry_pointers: &EntryPoints,
    ) -> (Vec<ExtensionProperties>, u32) {
        unsafe {
            // Counts number of extensions
            let mut extension_count: u32 = 0;

            let extension_count_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
                entry_pointers,
                null(),
                &mut extension_count,
                null_mut(),
            );
            // Manually allocates memory for extensions
            let extensions_vec_layout =
                Layout::array::<ExtensionProperties>(extension_count as usize).unwrap();

            let extensions_vec: *mut ExtensionProperties =
                alloc(extensions_vec_layout) as *mut ExtensionProperties;

            if extensions_vec.is_null() {
                panic!("Allocation for Extensions failed.");
            }
            // Gets extensions
            let extensions_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
                entry_pointers,
                null(),
                &mut extension_count,
                extensions_vec,
            );
            // Gets extensions in a returnable value
            let extensions_return_vec: Vec<ExtensionProperties> = Vec::from_raw_parts(
                extensions_vec,
                extension_count as usize,
                extension_count as usize,
            );
            // Checks if Instance Extension Counting was successful
            if extension_count_result == SUCCESS {
                println!("Extension Counting Successful")
            } else {
                panic!("Extension Checking Failed!");
            }
            // Checks if Instance Extension Receiving was successful
            if extensions_result == SUCCESS {
                println!("Extension Checking Successful");
                (extensions_return_vec, extension_count)
            } else {
                panic!("Extension Checking Failed!");
            }
        }
    }
    // Returns Vulkan Instance Layers
    pub unsafe fn return_instance_layers(
        entry_pointers: &EntryPoints,
        crash_on_failure: bool,
    ) -> (Vec<LayerProperties>, u32) {
        unsafe {
            // Counts number of Layers
            let mut layer_count: u32 = 0u32;

            let layer_counting_result: Result = EntryPoints::EnumerateInstanceLayerProperties(
                entry_pointers,
                &mut layer_count,
                null_mut(),
            );

            // Manually allocates memory for Layers
            let layer_vec_layout = Layout::array::<LayerProperties>(layer_count as usize).unwrap();

            let layers_vec: *mut LayerProperties = alloc(layer_vec_layout) as *mut LayerProperties;

            if layers_vec.is_null() {
                panic!("Allocation for Layers failed!");
            }

            // Inputs data into the Layer Vector
            let layers_vec_result: Result = EntryPoints::EnumerateInstanceLayerProperties(
                entry_pointers,
                &mut layer_count,
                layers_vec,
            );

            // Returns Layer Vector in a returnable format.
            let layers_return_vec: Vec<LayerProperties> =
                Vec::from_raw_parts(layers_vec, layer_count as usize, layer_count as usize);

            // Checks if Layer Counting was successful
            if layer_counting_result == SUCCESS {
                println!("Layer Counting Successful");
            } else {
                eprintln!("Layer Counting Failed");
            }

            // Checks if Layer Receiving was successful
            if layers_vec_result == SUCCESS {
                println!("Layer Checking Successful");
                return (layers_return_vec, layer_count);
            } else {
                eprintln!("Layer Checking Failed");
            }

            // Code comes to this point if the above wasn't successful
            // If the Developer does not request a Crash if the above fails
            // It simply returns an empty vector with a count of 0
            // Else, Panics
            if !crash_on_failure {
                (layers_return_vec, 0)
            } else {
                panic!("Layer Checking Failed, Crash Demanded");
            }
        }
    }

    pub fn return_filtered_layers<'a>(
        layers: &'a mut Vec<LayerProperties>,
        unwanted_layers: &[String],
    ) -> (&'a [LayerProperties], u32) {
        layers.retain(|layer| {
            !unwanted_layers
                .iter()
                .any(|unwanted| layer.layerName == static_c_char_array!(b"{unwanted}\0"))
        });

        let mut layer_count = 0;
        for i in &mut *layers {
            layer_count += 1;
        }
        return (layers.as_slice(), layer_count);
    }

    pub fn return_filtered_extensions<'a>(
        extensions: &'a mut Vec<ExtensionProperties>,
        unwanted_extensions: &[String],
    ) -> (&'a [ExtensionProperties], u32) {
        extensions.retain(|extension| {
            !unwanted_extensions
                .iter()
                .any(|unwanted| extension.extensionName == static_c_char_array!(b"{unwanted}\0"))
        });

        let mut extension_count = 0;
        for i in &mut *extensions {
            extension_count += 1;
        }
        return (extensions.as_slice(), extension_count);
    }
}
