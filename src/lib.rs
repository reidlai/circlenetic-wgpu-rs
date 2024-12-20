use std::fmt;
use wgpu;
use futures::executor::block_on;

pub trait WgpuRuntime {
    fn new() -> Self;
    fn get_instance(&self) -> &wgpu::Instance;
    fn get_adapters(&self) -> &Vec<wgpu::Adapter>;
    fn get_devices(&self) -> &Vec<wgpu::Device>;
    fn get_queues(&self) -> &Vec<wgpu::Queue>;
}

#[derive(Debug)]
pub struct Runtime {
    // wgpu instance
    instance: wgpu::Instance,
    adapters: Vec<wgpu::Adapter>,
    devices: Vec<wgpu::Device>,
    queues: Vec<wgpu::Queue>,
}

impl WgpuRuntime for Runtime {
    fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: Default::default(),
        });
        let adapters = Self::emulate_supported_adapters(&instance);
        let runtime_future = async {
            let mut devices = Vec::<wgpu::Device>::new();
            let mut queues = Vec::<wgpu::Queue>::new();
            for adapter in &adapters {
                match adapter.request_device(&wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },None,).await {
                    Ok((device, queue)) => {
                        devices.push(device);
                        queues.push(queue);
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Runtime {instance: instance, adapters, devices, queues}
        };
        block_on(runtime_future)
    }


    // Getter and Setter

    fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    fn get_adapters(&self) -> &Vec<wgpu::Adapter> {
        &self.adapters
    }

    fn get_devices(&self) -> &Vec<wgpu::Device> {
        &self.devices
    }

    fn get_queues(&self) -> &Vec<wgpu::Queue> {
        &self.queues
    }

}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime struct")
    }
}

impl Runtime {

    // Emulate supported adapters
    fn emulate_supported_adapters(instance: &wgpu::Instance) -> Vec<wgpu::Adapter> {
        instance.enumerate_adapters(wgpu::Backends::all())
    }
    
}