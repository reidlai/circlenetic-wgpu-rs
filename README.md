# circlenetic-wgpu-rs

A lightweight Rust wrapper for wgpu that provides a simplified runtime management system for WebGPU operations, with support for burn machine learning operations.

## Overview

circlenetic-wgpu-rs is a Rust library that abstracts the initialization and management of wgpu resources. It provides a clean interface for handling WebGPU instances, adapters, devices, and queues, with special support for burn neural network operations.

## Features

- Automatic GPU device discovery and initialization
- Support for multiple GPU adapters
- Safe wrapper around wgpu core functionality
- Simple trait-based interface
- Support for all wgpu backends
- Integration with burn for neural network operations
- Automatic WebGPU device selection for ML workloads

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
circlenetic-wgpu-rs = { git = "https://github.com/yourusername/circlenetic-wgpu-rs", branch = "main" }
# Or for a specific version/tag:
# circlenetic-wgpu-rs = { git = "https://github.com/yourusername/circlenetic-wgpu-rs", tag = "v0.1.0" }
burn = { version = "0.15.0", features = ["wgpu", "autodiff"] }
wgpu = "23.0.1"
```

# Usage

## Basic Runtime Usage
Here's a basic example of how to use the runtime:

```rust
use circlenetic_wgpu_rs::{Runtime, WgpuRuntime};

fn main() {
    // Create a new runtime instance
    let runtime = Runtime::new();
    
    // Get an available WebGPU device
    if let Some((index, wgpu_device)) = runtime.get_available_wgpudevice() {
        println!("Found GPU device at index: {}", index);
        // Use the device for computations...
    }
}
```

## Neural Network Example

```rust
use burn::backend::wgpu::{Wgpu, WgpuDevice};
use burn::tensor::Tensor;
use circlenetic_wgpu_rs::{Runtime, WgpuRuntime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize WebGPU runtime
    let runtime: Runtime = Runtime::new();
    
    // Get available WebGPU device
    let (index, wgpu_device) = runtime.get_available_wgpudevice()
        .ok_or("No available device found")?;
    
    // Create and use your neural network model
    let model = SimpleNet::new(&wgpu_device);
    // ... perform neural network operations
    Ok(())
}
```

# API Reference

## WgpuRuntime Trait

The main trait that defines the interface for the runtime system:

```rust
pub trait WgpuRuntime {
    fn new() -> Self;
    fn get_instance(&self) -> &wgpu::Instance;
    fn get_adapters(&self) -> &Vec<wgpu::Adapter>;
    fn get_devices(&self) -> &Vec<wgpu::Device>;
    fn get_queues(&self) -> &Vec<wgpu::Queue>;
    fn get_available_wgpudevice(&self) -> Option<(usize, WgpuDevice)>;
}
```

## Runtime Struct

The concrete implementation that provides:
 - Automatic wgpu instance initialization
 - GPU adapter discovery
 - Device and queue management
 - WebGPU device selection for burn operations
 - Polling and maintenance of GPU devices

# Requirements

 - Rust 1.x
 - wgpu compatible GPU
 - Compatible graphics drivers
 - For neural network operations: GPU with compute capabilities

# License

MIT License

# Issues

If you find any bugs or have feature requests, please create an issue on the GitHub repository.

This README.md provides:
1. A clear overview of what the library does
2. Installation instructions
3. Basic usage example
4. API documentation
5. Requirements
6. Placeholders for license and contribution guidelines
