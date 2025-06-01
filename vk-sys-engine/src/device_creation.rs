pub mod mod_device_creation {    
    use vk_sys::{
        NULL_HANDLE, PhysicalDevice, 
        PhysicalDeviceProperties, PhysicalDeviceFeatures, 
        InstancePointers, Instance
    };
    use core::mem::{zeroed};
    use core::ptr::{null_mut};
    /// Picks the Physical Device
    fn pick_physical_device(
        instance_ptrs: &InstancePointers,
        instance: &Instance,
    ) {
        let mut physical_device: PhysicalDevice = NULL_HANDLE.try_into().unwrap();

        let mut device_count: u32 = 0;
        unsafe{InstancePointers::EnumeratePhysicalDevices(
            instance_ptrs,
            *instance,
            &mut device_count,
            null_mut(),
        )};

        if device_count == 0 {
            panic!("[UNRECOVERABLE]: [ERROR]: Failed to find GPUs with Vulkan Support!");
        }

        let mut devices: Vec<PhysicalDevice> = Vec::new();

        let p_devices: *mut PhysicalDevice = devices.as_mut_ptr();
        unsafe {InstancePointers::EnumeratePhysicalDevices(
            instance_ptrs,
            *instance,
            &mut device_count,
            p_devices,
        )};

        for device in devices {
            if is_device_suitable(&*instance_ptrs, device) {
                physical_device = device;
                break;
            }
        }

        if physical_device == NULL_HANDLE.try_into().unwrap() {
            panic!("[ERROR]: Failed to find a suitable GPU!");
        }
    }

    /// Checks if the Physial Device can be used.
    fn is_device_suitable(
        instance_ptrs: &InstancePointers,
        device: PhysicalDevice
    ) -> bool {
        let mut device_properties: PhysicalDeviceProperties = unsafe{zeroed()};
        unsafe{InstancePointers::GetPhysicalDeviceProperties(
            instance_ptrs,
            device,
            &mut device_properties,
        )};

        let mut device_features: PhysicalDeviceFeatures = unsafe{zeroed()};
        unsafe{InstancePointers::GetPhysicalDeviceFeatures(
            instance_ptrs,
            device, 
            &mut device_features,
        )};

        return device_properties.deviceType == vk_sys::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU 
            | vk_sys::PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU;
    }

    /// Feature Planned to be made later
    fn rate_device_suitability(device: PhysicalDevice) {
        todo!()
    }
    /// Creates the Vulkan Device
    pub unsafe fn create_device() {todo!()}

    pub mod mod_queue_families {
        use vk_sys::{
            PhysicalDevice, 
        };
        struct QueueFamilyIndices {
            graphics_family: u32,
        }

        fn find_queue_families(
            device: PhysicalDevice,
        ) -> QueueFamilyIndices {
            todo!()
        }
    }
}    