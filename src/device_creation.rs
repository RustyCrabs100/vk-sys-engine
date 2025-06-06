pub mod mod_device_creation {
    use core::ffi::c_char;
    use core::mem::zeroed;
    use core::ptr::{null, null_mut};
    use std::alloc::{Layout, alloc};
    use vk_sys::{
        AllocationCallbacks, Device, DeviceCreateInfo, DevicePointers, DeviceQueueCreateInfo,
        ExtensionProperties, Instance, InstancePointers, LayerProperties, NULL_HANDLE,
        PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceProperties, Queue,
        QueueFamilyProperties, Result, STRUCTURE_TYPE_DEVICE_CREATE_INFO, SUCCESS,
    };
    /// Picks the Physical Device
    pub fn pick_physical_device(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
    ) -> PhysicalDevice {
        let mut physical_device: PhysicalDevice = NULL_HANDLE.try_into().unwrap();

        let mut device_count: u32 = 0;
        let p_device_count: *mut u32 = &mut device_count as *mut u32;
        let result_counter: Result = unsafe {
            InstancePointers::EnumeratePhysicalDevices(
                instance_ptrs,
                *instance,
                p_device_count,
                null_mut(),
            )
        };

        if device_count == 0 {
            panic!("[UNRECOVERABLE]: [ERROR]: Failed to find GPUs with Vulkan Support!");
        }

        let device_layout: Layout = Layout::array::<PhysicalDevice>(device_count as usize)
            .expect("Layout failure: Physical Devices");
        let device_vec: *mut PhysicalDevice =
            unsafe { alloc(device_layout) as *mut PhysicalDevice };

        if device_vec.is_null() {
            panic!("Alloation for Physical Device Vector failed!");
        }

        let result: Result = unsafe {
            InstancePointers::EnumeratePhysicalDevices(
                instance_ptrs,
                *instance,
                p_device_count,
                device_vec,
            )
        };

        let devices = unsafe {
            Vec::from_raw_parts(device_vec, device_count as usize, device_count as usize)
        };

        for device in devices {
            if is_device_suitable(instance_ptrs, device) {
                physical_device = device;
                break;
            }
        }

        if result_counter != SUCCESS {
            panic!("[ERROR]: Failed to Count Physical Devices!");
        }

        if result != SUCCESS {
            panic!("[ERROR]: Failed to Enumerate Physical devices!");
        }

        if physical_device == NULL_HANDLE.try_into().unwrap() {
            panic!("[ERROR]: Failed to find a suitable GPU!");
        } else {
            physical_device
        }
    }
    // !! ERROR!
    /// Checks if the Physial Device can be used.
    fn is_device_suitable(instance_ptrs: &InstancePointers, device: PhysicalDevice) -> bool {
        let mut device_properties_layout: Layout =
            unsafe { Layout::new::<PhysicalDeviceProperties>() };
        let p_device_properties: *mut PhysicalDeviceProperties =
            unsafe { alloc(device_properties_layout) as *mut PhysicalDeviceProperties };
        unsafe {
            InstancePointers::GetPhysicalDeviceProperties(
                instance_ptrs,
                device,
                p_device_properties,
            )
        };

        let device_features: PhysicalDeviceFeatures = return_device_features(instance_ptrs, device);

        let device_properties = unsafe { &*p_device_properties };

        QueueFamilyIndices::is_complete(find_queue_families(instance_ptrs, &device))
    }

    fn return_device_features(
        instance_ptrs: &InstancePointers,
        device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        let mut device_features: PhysicalDeviceFeatures = unsafe { zeroed() };
        let p_device_features: *mut PhysicalDeviceFeatures =
            &mut device_features as *mut PhysicalDeviceFeatures;
        unsafe {
            InstancePointers::GetPhysicalDeviceFeatures(instance_ptrs, device, p_device_features);
        }
        device_features
    }

    /// Feature Planned to be made later
    fn rate_device_suitability(device: PhysicalDevice) {
        todo!()
    }
    /// Contains the Queue Family Indices
    #[derive(Default, Debug, PartialEq, Copy, Clone)]
    struct QueueFamilyIndices {
        graphics_family: Option<u32>,
    }

    impl QueueFamilyIndices {
        pub fn is_complete(self) -> bool {
            if self.graphics_family.is_some() {
                return true;
            }
            false
        }
    }

    /// Find queue families
    fn find_queue_families(
        instance_ptrs: &InstancePointers,
        device: &PhysicalDevice,
    ) -> QueueFamilyIndices {
        let mut indices: QueueFamilyIndices = QueueFamilyIndices {
            graphics_family: None,
        };

        let mut queue_family_count: u32 = 0;
        unsafe {
            InstancePointers::GetPhysicalDeviceQueueFamilyProperties(
                instance_ptrs,
                *device,
                &mut queue_family_count,
                null_mut(),
            )
        };

        let layout_queue_families =
            Layout::array::<QueueFamilyProperties>(queue_family_count as usize).unwrap();
        let p_queue_families: *mut QueueFamilyProperties =
            unsafe { alloc(layout_queue_families) as *mut QueueFamilyProperties };
        unsafe {
            InstancePointers::GetPhysicalDeviceQueueFamilyProperties(
                instance_ptrs,
                *device,
                &mut queue_family_count,
                p_queue_families,
            )
        }

        let queue_families: Vec<QueueFamilyProperties> = unsafe {
            Vec::from_raw_parts(
                p_queue_families,
                queue_family_count as usize,
                queue_family_count as usize,
            )
        };

        let mut counter: u32 = 0;
        for queue_family in queue_families {
            if indices.is_complete() {
                break;
            }
            if queue_family.queueFlags & vk_sys::QUEUE_GRAPHICS_BIT != 0 {
                indices.graphics_family = Some(counter);
                break;
            }

            counter += 1;
        }

        indices
    }

    pub fn create_logical_device(
        instance_ptrs: &InstancePointers,
        device: &PhysicalDevice,
        device_layers: Vec<*const c_char>,
        device_layers_count: u32,
        device_extensions: Vec<*const c_char>,
        device_extension_count: u32,
        p_allocator: *const AllocationCallbacks,
    ) -> Device {
        let indices: QueueFamilyIndices = find_queue_families(instance_ptrs, device);
        let valid_graphics_family: u32 = match indices.graphics_family {
            Some(x) => x,
            None => panic!("Critical Step failed, create_logical_device"),
        };

        let queue_priority: f32 = 1.0f32;
        let p_queue_priority: *const f32 = &queue_priority as *const f32;

        let queue_create_info: DeviceQueueCreateInfo = DeviceQueueCreateInfo {
            sType: vk_sys::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            queueFamilyIndex: valid_graphics_family,
            queueCount: 1,
            pQueuePriorities: p_queue_priority,
        };
        let p_queue_create_info: *const DeviceQueueCreateInfo =
            &queue_create_info as *const DeviceQueueCreateInfo;

        let device_features: PhysicalDeviceFeatures =
            return_device_features(instance_ptrs, *device);

        let p_device_features: *const PhysicalDeviceFeatures =
            &device_features as *const PhysicalDeviceFeatures;

        let device_create_info: DeviceCreateInfo = DeviceCreateInfo {
            sType: STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            queueCreateInfoCount: queue_create_info.queueCount,
            pQueueCreateInfos: p_queue_create_info,
            enabledLayerCount: device_layers_count,
            ppEnabledLayerNames: device_layers.as_ptr(),
            enabledExtensionCount: device_extension_count,
            ppEnabledExtensionNames: device_extensions.as_ptr(),
            pEnabledFeatures: p_device_features,
        };

        let p_device_create_info: *const DeviceCreateInfo =
            &device_create_info as *const DeviceCreateInfo;

        let mut logical_device: Device = unsafe { zeroed() };
        let p_logical_device: *mut Device = &mut logical_device as *mut Device;
        let device_result: Result = unsafe {
            InstancePointers::CreateDevice(
                instance_ptrs,
                *device,
                p_device_create_info,
                p_allocator,
                p_logical_device,
            )
        };

        if device_result != SUCCESS {
            panic!("Logical Device Failed to be made!");
        }
        logical_device
    }

    pub fn return_device_layers(
        instance_ptrs: &InstancePointers,
        device: &PhysicalDevice,
        crash_if_fail: bool,
    ) -> (u32, Vec<LayerProperties>) {
        let mut device_layer_count: u32 = 0;
        let counter: vk_sys::Result = unsafe {
            InstancePointers::EnumerateDeviceLayerProperties(
                instance_ptrs,
                *device,
                &mut device_layer_count,
                null_mut(),
            )
        };

        let device_layers_layout =
            Layout::array::<LayerProperties>(device_layer_count as usize).unwrap();
        let device_layers: *mut LayerProperties =
            unsafe { alloc(device_layers_layout) as *mut LayerProperties };

        if device_layers.is_null() {
            panic!("Allocation for Device Layers failed");
        }

        let collector: vk_sys::Result = unsafe {
            InstancePointers::EnumerateDeviceLayerProperties(
                instance_ptrs,
                *device,
                &mut device_layer_count,
                device_layers,
            )
        };

        let device_layers_vec: Vec<LayerProperties> = unsafe {
            Vec::from_raw_parts(
                device_layers,
                device_layer_count as usize,
                device_layer_count as usize,
            )
        };

        if counter == SUCCESS {
            println!("Device Layer Counting Successful");
        } else {
            eprintln!("Device Layer Counting Failed");
        }

        if collector == SUCCESS {
            println!("Device Layer Collecting Successful.");
            return (device_layer_count, device_layers_vec);
        } else {
            eprintln!("Device Layer Collecting Failed!");
        }

        if !crash_if_fail {
            (0, device_layers_vec)
        } else {
            panic!("Developer Controlled Crash, Device Layers failed to load");
        }
    }

    pub fn return_device_extensions(
        instance_ptrs: &InstancePointers,
        device: &PhysicalDevice,
        device_layers: *const c_char,
    ) -> (u32, Vec<ExtensionProperties>) {
        let mut device_extension_count: u32 = 0;
        let counter: vk_sys::Result = unsafe {
            InstancePointers::EnumerateDeviceExtensionProperties(
                instance_ptrs,
                *device,
                device_layers,
                &mut device_extension_count,
                null_mut(),
            )
        };

        let device_extensions_layout =
            Layout::array::<ExtensionProperties>(device_extension_count as usize).unwrap();
        let device_extensions: *mut ExtensionProperties =
            unsafe { alloc(device_extensions_layout) as *mut ExtensionProperties };

        if device_extensions.is_null() {
            panic!("Allocation for Device Layers failed");
        }

        let collector: vk_sys::Result = unsafe {
            InstancePointers::EnumerateDeviceExtensionProperties(
                instance_ptrs,
                *device,
                device_layers,
                &mut device_extension_count,
                device_extensions,
            )
        };

        let device_extensions_vec: Vec<ExtensionProperties> = unsafe {
            Vec::from_raw_parts(
                device_extensions,
                device_extension_count as usize,
                device_extension_count as usize,
            )
        };

        if counter == SUCCESS {
            println!("Device Extension Counting Successful");
        } else {
            eprintln!("Device Extension Counting Failed");
        }

        if collector == SUCCESS {
            println!("Device Extension Collecting Successful.");
            (device_extension_count, device_extensions_vec)
        } else {
            panic!("Failed to collect Device Extensions!");
        }
    }

    pub fn create_graphics_queue(
        instance_ptrs: &InstancePointers,
        device_pointers: &DevicePointers,
        physical_device: &PhysicalDevice,
        logical_device: &Device,
    ) -> Queue {
        let indices: QueueFamilyIndices = find_queue_families(instance_ptrs, physical_device);
        let true_indices = match indices.graphics_family {
            Some(x) => x,
            None => panic!("Graphics Family Not Complete!"),
        };

        let mut graphics_queue: Queue = NULL_HANDLE.try_into().unwrap();
        let p_graphics_queue: *mut Queue = &mut graphics_queue as *mut Queue;
        unsafe {
            DevicePointers::GetDeviceQueue(
                device_pointers,
                *logical_device,
                true_indices,
                0,
                p_graphics_queue,
            );
        }

        return graphics_queue;
    }
}
