//! Basic Game Engine built in Rust.
//!
//! Provides a renderer and an editor for building games in Rust.
//! This Game Engine is currently not ready for any usage, and is still being developed.
//!
//! If you require a Game Engine that currently works, use Bevy.

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
use create_window::mod_window::*;
mod return_pfns;
use return_pfns::mod_return_pfns::{
    return_device_pointers, return_entry_points, return_instance_pointers,
};
mod vk_debugger;
use vk_debugger::mod_vk_debugger::{
    checking_validation_support, destroy_debug_messenger, return_allocation_callbacks,
    return_debug_messenger, return_validation, vk_debug_messenger_init,
};
mod device_creation;
mod instance_creation;
use device_creation::mod_device_creation::{
    create_logical_device, pick_physical_device, return_device_extensions, return_device_layers,
};
use instance_creation::mod_instance_creation::{
    create_instance, return_instance_extensions, return_instance_layers,
};
// Standard Library Imports
use core::ffi::c_char;
use std::thread;
use std::{ptr::null, sync::Arc};

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    AllocationCallbacks, DebugUtilsMessengerCreateInfoEXT, Device, DevicePointers, EntryPoints,
    ExtensionProperties, Instance, InstancePointers, LayerProperties, NULL_HANDLE, PhysicalDevice,
    Queue, SUCCESS,
};

use crate::device_creation::mod_device_creation::create_graphics_queue;

// Minimal Debugging Library Imports (mini_log Imports)
/// Defined to contain if debugging is enabled
pub const VALIDATION: bool = return_validation();
/// The VkSysEngine Struct allows you to manually define certain aspects of the game.
#[derive(Debug, Clone, PartialEq)]
pub struct VkSysEngine {
    /// Sets window width
    window_width: u32,
    /// Sets window height
    window_height: u32,
    /// Sets Highest Usable Vulkan Version
    vulkan_version: u32,
    /// Sets Game Engine Version
    engine_version: u32,
    /// Sets Game Version
    application_version: u32,
}

impl VkSysEngine {
    pub fn new(
        window_width: u32,
        window_height: u32,
        vulkan_version: u32,
        engine_version: u32,
        application_version: u32,
    ) -> Self {
        Self {
            window_width,
            window_height,
            vulkan_version,
            engine_version,
            application_version,
        }
    }
    /// Begins to run the game engine
    /// Logging using mini_log planned for the future
    pub fn run(&mut self) {
        AppWindow::run_engine_window(AppWindow::create_event_loop());
        let vulkan_lib: Library = unsafe { load_vulkan().expect("Unable to load Vulkan") };
        let allocation_callbacks: AllocationCallbacks = return_allocation_callbacks();
        let entry_points: Arc<EntryPoints> = Arc::new(unsafe { return_entry_points(&vulkan_lib) });

        let entry_points_copy: Arc<EntryPoints> = Arc::clone(&entry_points);
        // Spawning new thread for Instance Layer & Extension Counting & Checking
        let vk_info_handle = thread::spawn(move || {
            println!("Running Instance Extension Counting and Collecting in Secondary Thread");
            // Collects Instance Extensions and Extension Count
            let (instance_extensions, instance_extensions_count) =
                unsafe { return_instance_extensions(&entry_points_copy) };

            println!("Finished running Secondary Thread");
            // Returns Instance Extensions and the Counts.
            (instance_extensions, instance_extensions_count)
        });

        // Collects Instance Layers and Layer Count
        let (instance_layers, instance_layers_count) =
            unsafe { return_instance_layers(&entry_points, true) };

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
        let instance: Instance = create_instance(
            &entry_points,
            self.application_version,
            self.engine_version,
            self.vulkan_version,
            &allocation_callbacks,
            instance_extensions_count,
            instance_extensions_vec,
            instance_layers_count,
            instance_layers_vec,
            VALIDATION,
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
        let physical_device: PhysicalDevice = pick_physical_device(&instance_pointers, &instance);
        let (logical_device_layer_count, logical_device_layers): (u32, Vec<LayerProperties>) =
            return_device_layers(&instance_pointers, &physical_device, true);
        let (logical_device_extension_count, logical_device_extensions): (
            u32,
            Vec<ExtensionProperties>,
        ) = return_device_extensions(&instance_pointers, &physical_device, null());
        let logical_device_extension_names: Vec<*const c_char> = logical_device_extensions
            .iter()
            .map(|extensions| extensions.extensionName.as_ptr())
            .collect();
        let logical_device_layer_names: Vec<*const c_char> = logical_device_layers
            .iter()
            .map(|layer| layer.layerName.as_ptr())
            .collect();
        let logical_device: Device = create_logical_device(
            &instance_pointers,
            &physical_device,
            logical_device_layer_names,
            logical_device_layer_count,
            logical_device_extension_names,
            logical_device_extension_count,
            &allocation_callbacks as *const AllocationCallbacks,
        );
        let device_pointers: DevicePointers =
            unsafe { return_device_pointers(&instance_pointers, &logical_device) };
        let graphics_queue: Queue = create_graphics_queue(
            &instance_pointers,
            &device_pointers,
            &physical_device,
            &logical_device,
        );
        Self::main_loop();
        unsafe {
            Self::cleanup(
                vulkan_lib,
                &instance_pointers,
                &allocation_callbacks,
                instance,
                debug_messenger.into(),
                &device_pointers,
                logical_device,
            );
        }
    }
    /// Stub Implementation
    fn main_loop() {}

    /// Cleans up data Rust can't
    unsafe fn cleanup(
        lib: libloading::Library,
        instance_pointers: &vk_sys::InstancePointers,
        allocation_callbacks: &vk_sys::AllocationCallbacks,
        instance: vk_sys::Instance,
        debug_messenger: vk_sys::DebugUtilsMessengerEXT,
        device_pointers: &vk_sys::DevicePointers,
        logical_device: vk_sys::Device,
    ) {
        unsafe {
            let allocation = allocation_callbacks as *const vk_sys::AllocationCallbacks;
            vk_sys::DevicePointers::DestroyDevice(device_pointers, logical_device, allocation);
            if VALIDATION {
                crate::destroy_debug_messenger(
                    instance_pointers,
                    &instance,
                    debug_messenger,
                    allocation,
                    VALIDATION,
                );
            }
            vk_sys::InstancePointers::DestroyInstance(instance_pointers, instance, allocation);
            let _ = crate::close_vulkan(lib);
        }
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
    let mut game_engine: VkSysEngine = VkSysEngine::new(
        800,
        600,
        make_version(1, 3, 296, 0),
        make_version(0, 1, 0, 0),
        make_version(0, 1, 0, 0),
    );
    game_engine.run();
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
