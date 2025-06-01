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
    return_debug_messenger, return_validation, vk_debug_messenger_init,
};
mod device_creation;
mod instance_creation;
use instance_creation::mod_instance_creation::{
    create_instance, return_instance_extensions, return_instance_layers,
};
// Standard Library Imports
use core::ffi::c_char;
use std::sync::Arc;
use std::thread;

// Libloading Imports (Library Loading Imports)
use libloading::Library;

// Minimal Vulkan Overhead Imports
use vk_sys::{
    AllocationCallbacks, DebugUtilsMessengerCreateInfoEXT, EntryPoints, Instance, InstancePointers,
    NULL_HANDLE, SUCCESS,
};

// Minimal Debugging Library Imports (mini_log Imports)
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
    pub fn new(
        window_width: usize,
        window_height: usize,
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
        let device = 0;
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
    /// Stub Implementation
    fn main_loop() {}

    /// Cleans up data Rust can't
    unsafe fn cleanup(
        lib: Library,
        instance_pointers: &InstancePointers,
        allocation_callbacks: &AllocationCallbacks,
        instance: Instance,
        debug_messenger: vk_sys::DebugUtilsMessengerEXT,
    ) {
        unsafe {
            let allocation = allocation_callbacks as *const AllocationCallbacks;
            if VALIDATION {
                destroy_debug_messenger(
                    instance_pointers,
                    &instance,
                    debug_messenger,
                    allocation,
                    VALIDATION,
                );
            }
            InstancePointers::DestroyInstance(instance_pointers, instance, allocation);
            let _ = close_vulkan(lib);
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
        make_version(1, 0, 0, 0),
        make_version(0, 1, 0, 0),
        make_version(0, 1, 0, 0),
    );
    game_engine.run();
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
