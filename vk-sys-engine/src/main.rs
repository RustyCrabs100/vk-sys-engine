//! Basic Game Engine built in Rust.
//!
//! Provides a renderer and an editor for building games in Rust.
//! This Game Engine is currently not ready for any usage, and is still being developed.
//!
//! If you require a Game Engine that currently works, use Bevy.

// On your next commit, make sure to replace
// Library::get() with vkGetInstanceProcAddr
// and vkGetDeviceProcAddr, as those are for
// properly getting vulkan function pointers.
// Library::get() should only be used to load
// in vulkan and to load in
// vkGetInstanceProcAddr
// How to do this:
// For core Vulkan features:
// Use Library::get() from libloading
// For Instance/Device Function Pointers:
// Use Library::get() to get:
// vkGetInstanceProcAddr, and
// vkGetDeviceProcAddr.
// Then, use the proper function to call the
// Instance or Device Function Pointer

// Stopping Rust Compiler from complaning
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_mut)]
#![warn(unused_variables)]

// Personal file loading
mod vulkan_loader;
use vulkan_loader::mod_vulkan_loader::{close_vulkan, load_vulkan};
mod utils;
use utils::mod_utils::make_version;
mod create_window;
use create_window::mod_window::window_creation;
mod return_pfns;
use return_pfns::mod_return_pfns::{return_entry_points, return_instance_pointers}; // return_instance_pointers};
mod vk_debugger;
use vk_debugger::mod_vk_debugger::{
    checking_validation_support, destroy_debug_messenger, return_allocation_callbacks,
    return_debug_messenger, return_validation, vk_debug_callback, vk_debug_messenger_init,

};
// Standard Library Imports
use core::ffi::{c_char, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};
use std::alloc::{Layout, alloc};
use std::sync::Arc;
use std::thread;

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    AllocationCallbacks, ApplicationInfo, DebugUtilsMessengerCreateInfoEXT, EntryPoints,
    ExtensionProperties, Instance, InstanceCreateInfo, InstancePointers, LayerProperties, Result,
    STRUCTURE_TYPE_APPLICATION_INFO, STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS, NULL_HANDLE,
    PhysicalDevice, PhysicalDeviceProperties, PhysicalDeviceFeatures, 
};

// Minimal Debugging Library Imports (mini_log Imports)
use mini_log::{Logger, LoggingType};
/// Defined to contain if debugging is enabled
pub const VALIDATION: bool = return_validation();
/// The VkSysEngine Struct allows you to manually define certain aspects of the game.
pub struct VkSysEngine {
    /// Sets window width
    window_width: usize,
    /// Sets window height
    window_height: usize,
    /// Sets Highest Usable Vulkan Version
    vulkan_version: u32,
    /// Sets Game Engine Version
    engine_version: u32,
    /// Sets Game Version
    application_version: u32,
}

impl VkSysEngine {
    /// Begins to run the game engine
    /// Logging using mini_log planned for the future
    pub fn run(&mut self) {
        window_creation(self.window_height, self.window_width);
        let vulkan_lib: Library = unsafe { load_vulkan().expect("Unable to load Vulkan") };
        let allocation_callbacks: AllocationCallbacks = return_allocation_callbacks();
        let entry_points: Arc<EntryPoints> = Arc::new(unsafe { return_entry_points(&vulkan_lib) });

        let entry_points_copy: Arc<EntryPoints> = Arc::clone(&entry_points);
        // Spawning new thread for Instance Layer & Extension Counting & Checking
        let vk_info_handle = thread::spawn(move || {
            println!("Running Instance Extension Counting and Collecting in Secondary Thread");
            // Collects Instance Extensions and Extension Count
            let (instance_extensions, instance_extensions_count) =
                unsafe { Self::return_instance_extensions(&entry_points_copy) };

            println!("Finished running Secondary Thread");
            // Returns Instance Extensions and the Counts.
            (instance_extensions, instance_extensions_count)
        });

        // Collects Instance Layers and Layer Count
        let (mut instance_layers, instance_layers_count) =
            unsafe { Self::return_instance_layers(&entry_points, true) };

        // Collects info from secondary thread
        let (mut instance_extensions, instance_extensions_count) = vk_info_handle.join().unwrap();

        if !checking_validation_support(&instance_layers) && !VALIDATION {
            println!("Validation not Supported");
            instance_extensions
                .retain(|f| f.extensionName != static_c_char_array!(b"VK_EXT_debug_utils\0"));
        }

        let instance_layers_vec: Vec<*const c_char> = instance_layers
            .iter()
            .map(|f| f.layerName.as_ptr())
            .collect::<Vec<_>>();

        let instance_extensions_vec: Vec<*const c_char> = instance_extensions
            .iter()
            .map(|f| f.extensionName.as_ptr())
            .collect::<Vec<_>>();
        //let instance_pointers: InstancePointers = unsafe { return_instance_pointers(&vulkan_lib) };
        let instance: Instance = Self::create_instance(
            &entry_points,
            self.application_version,
            self.engine_version,
            self.vulkan_version,
            &allocation_callbacks,
            instance_extensions_count,
            instance_extensions_vec,
            instance_layers_count,
            instance_layers_vec,
        );
        let instance_pointers: InstancePointers =
            unsafe { return_instance_pointers(&vulkan_lib, Some(&instance)) };
        let debug_messenger_uninit: u64 = NULL_HANDLE;    
        let debug_messenger = return_debug_messenger(
            &instance_pointers,
            &instance,
            &vk_debug_messenger_init() as *const DebugUtilsMessengerCreateInfoEXT,
            &allocation_callbacks as *const AllocationCallbacks,
            &debug_messenger_uninit as *const vk_sys::DebugUtilsMessengerEXT,
            VALIDATION,
        );

        if debug_messenger == SUCCESS {
            println!("Debug Messenger Setup Complete");
        }
        Self::main_loop();
        unsafe {
            Self::cleanup(
                vulkan_lib,
                &instance_pointers,
                &allocation_callbacks,
                instance,
                debug_messenger.into(),
            );
        }
    }
    /// Creates Vulkan Instance
    fn create_instance(
        entry_pointers: &EntryPoints,
        application_version: u32,
        engine_version: u32,
        vulkan_version: u32,
        allocation_callbacks: &AllocationCallbacks,
        extension_count: u32,
        extensions: Vec<*const i8>,
        layer_count: u32,
        layers: Vec<*const i8>,
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
        if VALIDATION {
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
    /// Picks the Physical Device
    fn pick_physical_device(
        &self, 
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

        let mut p_devices: *mut PhysicalDevice = devices.as_mut_ptr();
        unsafe {InstancePointers::EnumeratePhysicalDevices(
            instance_ptrs,
            *instance,
            &mut device_count,
            p_devices,
        )};

        for device in devices {
            if Self::is_device_suitable(&*instance_ptrs, device) {
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
    fn create_device() {todo!()}
    /// Returns Vulkan Instance Extensions
    unsafe fn return_instance_extensions(
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
    unsafe fn return_instance_layers(
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

    fn return_filtered_layers<'a>(
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

    fn return_filtered_extensions<'a>(
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

    fn main_loop() {}

    // Cleans up data Rust can't
    unsafe fn cleanup(
        lib: Library,
        instance_pointers: &InstancePointers,
        allocation_callbacks: &AllocationCallbacks,
        instance: Instance,
        debug_messenger: vk_sys::DebugUtilsMessengerEXT,
    ) {
        let allocation = allocation_callbacks as *const AllocationCallbacks;
        if VALIDATION {
            destroy_debug_messenger(instance_pointers, &instance, debug_messenger, allocation, VALIDATION);
        }
        InstancePointers::DestroyInstance(instance_pointers, instance, allocation);
        let _ = close_vulkan(lib);
    }
}

impl Default for VkSysEngine {
    fn default() -> VkSysEngine {
        VkSysEngine {
            window_height: 600,
            window_width: 800,
            vulkan_version: make_version(1, 0, 0, 0),
            engine_version: make_version(0, 1, 0, 0),
            application_version: make_version(0, 1, 0, 0),
        }
    }
}

fn main() {
    let mut game_engine: VkSysEngine = VkSysEngine {
        window_height: 600,
        window_width: 800,
        vulkan_version: make_version(1, 3, 296, 0),
        engine_version: make_version(0, 1, 0, 0),
        application_version: make_version(0, 1, 0, 0),
    };
    VkSysEngine::run(&mut game_engine);
    println!("Working so far");
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::vulkan_loader::mod_vulkan_loader::return_get_instance_proc_addr_pfn;
    use core::ffi::c_void;
    use core::ptr::null;

    #[test]
    fn validation_test() {
        if cfg!(debug_assertions) {
            assert_eq!(VALIDATION, true);
        } else {
            assert_eq!(VALIDATION, false);
        }
    }
}

// https://github.com/nagisa/rust_libloading/ - libloading Github
// https://github.com/emoon/rust_minifb - minifb Github
// The vk-sys github is not directly linkable, but we could use the vulkano github (since vulkano-rs is the dev)
// https://github.com/vulkano-rs/vulkano
