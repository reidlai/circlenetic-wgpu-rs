# circlenetic-wgpu-rs

A lightweight Rust wrapper for wgpu that provides a simplified runtime management system for WebGPU operations.

## Overview

circlenetic-wgpu-rs is a Rust library that abstracts the initialization and management of wgpu resources. It provides a clean interface for handling WebGPU instances, adapters, devices, and queues.

## Features

- Automatic GPU device discovery and initialization
- Support for multiple GPU adapters
- Safe wrapper around wgpu core functionality
- Simple trait-based interface
- Support for all wgpu backends

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
circlenetic-wgpu-rs = "0.1.0"
```

# Usage

Here's a basic example of how to use the runtime:

```rust
use circlenetic_wgpu_rs::{Runtime, WgpuRuntime};

fn main() {
    // Create a new runtime instance
    let runtime = Runtime::new();
    
    // Access GPU resources
    let instance = runtime.get_instance();
    let adapters = runtime.get_adapters();
    let devices = runtime.get_devices();
    let queues = runtime.get_queues();
    
    // Work with the GPU resources...
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
}
```

## Runtime Struct

The concrete implementation that provides GPU resource management:
Automatically initializes wgpu instance
Discovers available GPU adapters
Creates devices and command queues
Provides access to all GPU resources

# Requirements

 - Rust 1.x
 - wgpu compatible GPU
 - Compatible graphics drivers

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
