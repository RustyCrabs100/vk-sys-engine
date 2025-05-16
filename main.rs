// Personal file loading
mod vulkan_loader;
use vulkan_loader::mod_vulkan_loader::{close_vulkan, load_vulkan, return_vulkan_item};
mod utils;
use utils::mod_utils::{make_version, parse_version};
mod create_window;
use create_window::mod_window::window_creation;
mod return_pfns;
use return_pfns::mod_return_pfns::{return_entry_points, return_instance_pointers};
// Standard Library Imports
use std::mem::zeroed;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    ApplicationInfo, EntryPoints, ExtensionProperties, Instance,
    InstanceCreateInfo, InstancePointers, Result, STRUCTURE_TYPE_APPLICATION_INFO,
    STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS, AllocationCallbacks
};

struct VkSysEngine;

impl VkSysEngine {
    pub fn run(&mut self) {
        window_creation(800, 600);
        let vulkan_lib: Library = unsafe { load_vulkan().expect("Unable to load Vulkan") };
        let entry_points: EntryPoints = unsafe { return_entry_points(&vulkan_lib) };
        let instance_pointers: InstancePointers = unsafe { return_instance_pointers(&vulkan_lib) };
        let instance: Instance = Self::create_instance(entry_points);
        Self::main_loop();
        unsafe {
            Self::cleanup(vulkan_lib, instance_pointers, null(), instance);
        }
    }

    fn create_instance(entry_pointers: EntryPoints) -> Instance {
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
            enabledLayerCount: 0,
            ppEnabledLayerNames: null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: null(),
        };

        let mut instance: Instance = unsafe { zeroed() };

        let instance_pointer: *mut Instance = &mut instance;

        let result: Result = unsafe {
            EntryPoints::CreateInstance(
                &entry_pointers,
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
        entry_pointers: EntryPoints,
    ) -> (Vec<ExtensionProperties>, u32) {
        let mut extension_count: u32 = 0;

        let extension_count_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
            &entry_pointers,
            null(),
            extension_count as *mut u32,
            null_mut(),
        );

        let mut extensions_vec: Vec<ExtensionProperties> = Vec::with_capacity(20);

        let extensions_result: Result = EntryPoints::EnumerateInstanceExtensionProperties(
            &entry_pointers,
            null(),
            extension_count as *mut u32,
            extensions_vec.as_mut_ptr(),
        );

        if extension_count_result == SUCCESS {
            println!("Extension Checking Successful");
        } else {
            panic!("Extension Checking Failed!");
        }

        return (extensions_vec, extension_count);
    }

    fn main_loop() {}

    unsafe fn cleanup(
        lib: Library,
        instance_pointers: InstancePointers,
        allocation_callbacks: *const AllocationCallbacks,
        instance: Instance,
    ) {
        let _ = close_vulkan(lib);
        InstancePointers::DestroyInstance(&instance_pointers, instance, allocation_callbacks);
    }
}

fn main() {
    VkSysEngine.run();
    println!("Working so far");
}
