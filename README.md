# Note 
This repository is no longer being maintained. The new version of this is using a different crate for the Vulkan API. The repository is called ash-engine, and will have different focuses than this repository.

# vk-sys-engine
A small game engine written in rust using vk-sys as it's Graphics API

## Promises 
 * Fully Complete Renderer (Will not be optimized fully)
 * A seperate application to create games in (Like Godot or Unity)


### Why are these your promises? 
This is just a *small* project I'm working on with a friend, so expect nothing except that it works. 

## Plans for Version 1.0
 1.  Finish setting up the Renderer
 2.  Add UI to the engine.
 3.  Add support for Linux and MacOS
 4.  Begin properly optimizing the Renderer
 5.  Add OpenGL support

### *Potential* Plans for Version 1.0
 1. Add C++ support
    > This is because of how common C++ is in Game Development
 2. Add support for DirectX12 and 11
    > This is because DX12 and DX11 are nearly the same as Vulkan, and wouldn't add support for anything new. Only advantage over Vulkan for DX12 and DX11 is that they are more stable then Vulkan in terms of FPS.   


## Dependencies:

These are **REQUIRED ONLY** if your going to ***COMPILE*** the project. 

### Rendering 
 1. The Vulkan SDK: functions are dynamically loaded, meaning you'll need the VulkanSDK by LunarG to run this. 
    > This is because vk_sys doesn't automatically call the Vulkan Function Pointers, 
    unlike vulkano for example.
 2. The Slang Shader Language Compiler
    > This is for compiling the Slang Shader Language that we use for writing our Shaders.
 3. vk-sys: this is an **EXTREMELY** light wrapper around vulkan. 
### Library Loading
 4. libloading: this is used to dynamically load in vulkan and it's function pointers. 
### Windowing
 5. async-winit: this is used to create the window and handle input from the keyboard & mouse asynchronously.
    > This is because minifb doesn't provide a hwnd or an hinstance for vulkan, meaning that minifb can't be used for this project. This does increase the amount of bloat significantly. 
    >> Winit is not Async, meaning we can't use that. It is still technically asynchronous. 
 6. raw-window-handle: this is used to get a window's handles.
### Logging
 7. mini_log: this is used to log the program.
    > mini_log currently isn't being used, but is planned to be.
### Asynchronous Runtime
 8. smol: this is used to be a lightweight async runtime.
 9. futures: this provides core features for our async runtime.

 These are **REQUIRED NO MATTER WHAT** 
  * Vulkan Drivers for your GPU  

## What can we do with this engine? 

In this current version, absolutely **NOTHING**