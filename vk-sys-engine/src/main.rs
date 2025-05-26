//! Basic Game Engine built in Rust.
//!
//! Provides a renderer and an editor for building games in Rust.
//! This Game Engine is currently not ready for any usage, and is still being developed.
//!
//! If you require a Game Engine that currently works, use Bevy.

// On your next commit, make sure to replace Library::get() with vkGetInstanceProcAddr and vkGetDeviceProcAddr, as those are for properly getting vulkan function pointers. Library::get() should only be used to load in vulkan and to load in vkGetInstanceProcAddr

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
use vk_debugger::mod_vk_debugger::{
    checking_validation_support, return_allocation_callbacks, return_validation,
};
// Standard Library Imports
use core::ffi::c_char;
use core::mem::zeroed;
use core::ptr::{null, null_mut};
use std::alloc::{Layout, alloc};
use std::sync::Arc;
use std::thread;

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    AllocationCallbacks, ApplicationInfo, EntryPoints, ExtensionProperties, Instance,
    InstanceCreateInfo, LayerProperties, Result, STRUCTURE_TYPE_APPLICATION_INFO,
    STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS,
};

const VALIDATION: bool = return_validation();
/// The VkSysEngine Struct allows you to manually define certain aspects of the game.
struct VkSysEngine {
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
        let (mut instance_extensions, instance_extensions_count) =
            vk_info_handle.join().unwrap();

        if !checking_validation_support(&instance_layers) {
            instance_layers
                .retain(|f| f.layerName != static_c_char_array!(b"VK_LUNARG_KHRONOS_validation\0"));
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
        Self::main_loop();
        unsafe {
            Self::cleanup(vulkan_lib); //instance_pointers, null(), instance);
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

            let layers_vec: *mut LayerProperties =
                alloc(layer_vec_layout) as *mut LayerProperties;

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
        //instance_pointers: InstancePointers,
        //allocation_callbacks: *const AllocationCallbacks,
        //instance: Instance,
    ) {
        // InstancePointers::DestroyInstance(&instance_pointers, instance, allocation_callbacks);
        let _ = close_vulkan(lib);
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
    use core::ffi::c_void;

    #[test]
    fn validation_test() {
        if cfg!(debug_assertions) {
            assert_eq!(VALIDATION, true);
        } else {
            assert_eq!(VALIDATION, false);
        }
    }

    #[test]
    fn macro_dummy_test_full() {
        let dummy_fn: extern "system" fn(*mut c_void, *const c_void) -> *mut c_void = vk_dummy_pfn_creator!(fn_utils, (a: *mut c_void, b: *const c_void), *mut c_void, null_mut());
    }

    #[test]
    fn macro_dummy_test_nret() {
        let dummy_fn: extern "system" fn(*mut c_void, *const c_void) =
            vk_dummy_pfn_creator!(fn_utils, (a: *mut c_void, b: *const c_void));
    }
}
