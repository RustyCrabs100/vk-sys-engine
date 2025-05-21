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
use return_pfns::mod_return_pfns::return_entry_points; // return_instance_pointers};
mod vk_debugger;
use vk_debugger::mod_vk_debugger::return_validation;
// Standard Library Imports
use core::ptr::{null, null_mut};
use core::mem::zeroed;
use core::ffi::c_char;

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    ApplicationInfo, EntryPoints, ExtensionProperties, Instance,
    InstanceCreateInfo, LayerProperties, Result, STRUCTURE_TYPE_APPLICATION_INFO,
    STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS,
};

const VALIDATION: bool = return_validation();

struct VkSysEngine;

impl VkSysEngine {
    pub fn run(&mut self) {
        window_creation(800, 600);
        let vulkan_lib: Library = unsafe { load_vulkan().expect("Unable to load Vulkan") };
        let entry_points: EntryPoints = unsafe { return_entry_points(&vulkan_lib) };
        let (instance_extensions, instance_extensions_count): (Vec<ExtensionProperties>, u32) =
            unsafe { Self::return_instance_extensions(&entry_points) };
        let (instance_layers, instance_layers_count): (Vec<LayerProperties>, u32) =
            unsafe { Self::return_instance_layers(&entry_points, true) };

        let instance_layers_vec: Vec<*const c_char> = instance_layers
            .iter()
            .map(|f| f.layerName.as_ptr())
            .collect::<Vec<_>>();    

        let instance_extensions_vec : Vec<*const c_char> = instance_extensions
            .iter()
            .map(|f| f.extensionName.as_ptr())
            .collect::<Vec<_>>();
        //let instance_pointers: InstancePointers = unsafe { return_instance_pointers(&vulkan_lib) };
        let instance: Instance = Self::create_instance(
            &entry_points,
            instance_extensions_count,
            instance_extensions_vec,
            instance_layers_count,
            instance_layers_vec,
        );
        Self::main_loop();
        unsafe {
            Self::cleanup(vulkan_lib); //instance_pointers, null(), instance);
        }
    }

    fn create_instance(
        entry_pointers: &EntryPoints,
        extension_count: u32,
        extensions: Vec<*const i8>,
        layer_count: u32,
        layers: Vec<*const i8>,
    ) -> Instance {
        let application_name_wrapper: [c_char; 256] = static_c_char_array!(b"Rust Game Engine\0");
        let engine_name_wrapper: [c_char; 256] = static_c_char_array!(b"Rustic\0");

        let app_info: ApplicationInfo = ApplicationInfo {
            sType: STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: application_name_wrapper.as_ptr(),
            applicationVersion: make_version(0, 1, 0, 0),
            pEngineName: engine_name_wrapper.as_ptr(),
            engineVersion: make_version(0, 1, 0, 0),
            apiVersion: make_version(1, 3, 296, 0),
        };

        let application_info_wrapper: *const ApplicationInfo = &app_info;

        let instance_create_info: InstanceCreateInfo = InstanceCreateInfo {
            sType: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: application_info_wrapper,
            enabledLayerCount: layer_count,
            ppEnabledLayerNames: layers.as_ptr(),
            enabledExtensionCount: extension_count,
            ppEnabledExtensionNames: extensions.as_ptr(),
        };

        let mut instance: Instance = unsafe { zeroed() };

        let instance_pointer: *mut Instance = &mut instance;

        let result: Result = unsafe {
            EntryPoints::CreateInstance(
                entry_pointers,
                &instance_create_info,
                null(),
                instance_pointer,
            )
        };

        if result == SUCCESS {
            println!("vkCreateInstance correctly Initalized");
        } else {
            panic!("vkCreateInstance failed, can't continue");
        }

        return instance;
    }

    unsafe fn return_instance_extensions(
        entry_pointers: &EntryPoints,
    ) -> (Vec<ExtensionProperties>, u32) { unsafe {
        let mut extension_count: u32 = 0;

        let extension_count_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
            entry_pointers,
            null(),
            &mut extension_count,
            null_mut(),
        );

        let mut extensions_vec: Vec<ExtensionProperties> = Vec::with_capacity(extension_count as usize);
        extensions_vec.set_len(extension_count as usize);

        let extensions_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
            entry_pointers,
            null(),
            &mut extension_count,
            extensions_vec.as_mut_ptr(),
        );

        if extension_count_result == SUCCESS {
            if extensions_result == SUCCESS {
                println!("Extension Checking Successful");
                return (extensions_vec, extension_count);
            } else {
                panic!("Extension Checking Failed!");
            }
        } else {
            panic!("Extension Checking Failed!");
        }
    }}

    unsafe fn return_instance_layers(
        entry_pointers: &EntryPoints,
        crash_on_failure: bool,
    ) -> (Vec<LayerProperties>, u32) { unsafe {
        let mut layer_count: u32 = 0u32;

        let layer_counting_result: Result = EntryPoints::EnumerateInstanceLayerProperties(
            entry_pointers,
            &mut layer_count,
            null_mut(),
        );

        let mut layers_vec: Vec<LayerProperties> = Vec::with_capacity(layer_count as usize);
        layers_vec.set_len(layer_count as usize);

        let layers_vec_result: Result = EntryPoints::EnumerateInstanceLayerProperties(
            entry_pointers,
            &mut layer_count,
            layers_vec.as_mut_ptr(),
        );

        if layers_vec_result == SUCCESS {
            println!("Layer Checking Successful");
            return (layers_vec, layer_count);
        } else {
            if !crash_on_failure {
                eprintln!("Layer Checking Failed");
                return (layers_vec, 0);
            } else {
                panic!("Layer Checking Failed, Crash Demanded");
            }
        }
    }}

    fn main_loop() {}

    unsafe fn cleanup(
        lib: Library,
        //instance_pointers: InstancePointers,
        //allocation_callbacks: *const AllocationCallbacks,
        //instance: Instance,
    ) {
        let _ = close_vulkan(lib);
        // InstancePointers::DestroyInstance(&instance_pointers, instance, allocation_callbacks);
    }
}

fn main() {
    VkSysEngine.run();
    println!("Working so far");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_test() {
        if cfg!(debug_assertions) {
            assert_eq!(VALIDATION, true);
        } else {
            assert_eq!(VALIDATION, false);
        }
    }
}
