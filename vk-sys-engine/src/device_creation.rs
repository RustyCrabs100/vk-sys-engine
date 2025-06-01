pub mod mod_device_creation {
    use core::mem::zeroed;
    use core::ptr::null_mut;
    use vk_sys::{
        Instance, InstancePointers, NULL_HANDLE, PhysicalDevice, PhysicalDeviceFeatures,
        PhysicalDeviceProperties, QueueFamilyProperties,
    };
    /// Picks the Physical Device
    fn pick_physical_device(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
        device: &PhysicalDevice,
    ) -> PhysicalDevice {
        let mut physical_device: PhysicalDevice = NULL_HANDLE.try_into().unwrap();

        let mut device_count: u32 = 0;
        unsafe {
            InstancePointers::EnumeratePhysicalDevices(
                instance_ptrs,
                *instance,
                &mut device_count,
                null_mut(),
            )
        };

        if device_count == 0 {
            panic!("[UNRECOVERABLE]: [ERROR]: Failed to find GPUs with Vulkan Support!");
        }

        let mut devices: Vec<PhysicalDevice> = Vec::new();

        let p_devices: *mut PhysicalDevice = devices.as_mut_ptr();
        unsafe {
            InstancePointers::EnumeratePhysicalDevices(
                instance_ptrs,
                *instance,
                &mut device_count,
                p_devices,
            )
        };

        for device in devices {
            if is_device_suitable(&*instance_ptrs, device) {
                physical_device = device;
                break;
            }
        }

        if physical_device == NULL_HANDLE.try_into().unwrap() {
            panic!("[ERROR]: Failed to find a suitable GPU!");
        } else {
            physical_device
        }
    }

    /// Checks if the Physial Device can be used.
    fn is_device_suitable(instance_ptrs: &InstancePointers, device: PhysicalDevice) -> bool {
        let mut device_properties: PhysicalDeviceProperties = unsafe { zeroed() };
        unsafe {
            InstancePointers::GetPhysicalDeviceProperties(
                instance_ptrs,
                device,
                &mut device_properties,
            )
        };

        let mut device_features: PhysicalDeviceFeatures = unsafe { zeroed() };
        unsafe {
            InstancePointers::GetPhysicalDeviceFeatures(instance_ptrs, device, &mut device_features)
        };

        return device_properties.deviceType
            == vk_sys::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                | vk_sys::PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU
            && QueueFamilyIndices::is_complete(find_queue_families(&*instance_ptrs, &device));
    }

    /// Feature Planned to be made later
    fn rate_device_suitability(device: PhysicalDevice) {
        todo!()
    }
    /// Creates the Vulkan Device
    pub unsafe fn create_device() {
        todo!()
    }
    /// Contains the Queue Family Indices
    #[derive(Default, Debug, PartialEq, Copy, Clone)]
    struct QueueFamilyIndices {
        graphics_family: Option<u32>,
    }

    impl QueueFamilyIndices {
        pub fn is_complete(self) -> bool {
            if self.graphics_family != None {
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
        let p_queue_family_count: *mut u32 = queue_family_count as *mut u32;
        unsafe {
            InstancePointers::GetPhysicalDeviceQueueFamilyProperties(
                instance_ptrs,
                *device,
                p_queue_family_count,
                null_mut(),
            )
        }

        let mut queue_families: Vec<QueueFamilyProperties> = Vec::new();
        let p_queue_families: *mut QueueFamilyProperties = queue_families.as_mut_ptr();
        unsafe {
            InstancePointers::GetPhysicalDeviceQueueFamilyProperties(
                instance_ptrs,
                *device,
                p_queue_family_count,
                p_queue_families,
            )
        }

        let mut counter: u32 = 0;
        for queue_family in queue_families {
            if *&indices.is_complete() {
                break;
            }
            if queue_family.queueFlags == vk_sys::QUEUE_GRAPHICS_BIT {
                indices.graphics_family = Some(counter);
            }

            counter += 1;
        }

        indices
        
    }
}
