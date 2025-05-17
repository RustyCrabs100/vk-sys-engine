# vk-sys-engine
A small game engine written in rust using vk-sys as it's Graphics API

## Promises 
It will work. That's it. 

### Why are these your promises? 
This is just a *small* project I'm working on with a friend, so expect nothing except that it works. 

## Plans for the future 
If this is something I want to continue working on, I *might* just continue working on it. Maybe be ambitious and be competitive, at least with other Rust Game Engines. I might convert this into something else if my plans for this engine changes. 

## Dependencies:

The Vulkan SDK: functions are dynamically loaded, meaning you'll need the VulkanSDK by LunarG to run this. \
vk-sys: this is an **EXTREMELY** light wrapper around vulkan. \
libloading: this is used to dynamically load in vulkan and it's function pointers. \
minifb: this is used to create the window. \