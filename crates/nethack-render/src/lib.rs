//! wgpu-based rendering engine for NetHack
//! 
//! This module provides a unified rendering interface that works on both
//! desktop (via winit) and web (WASM/WebGPU).

pub mod renderer;

pub use renderer::WgpuRenderer;
