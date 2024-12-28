use std::fmt;
use wgpu;
use futures::executor::block_on;

pub trait WgpuRuntime {
    fn new() -> Self;
    fn get_instance(&self) -> &wgpu::Instance;
    fn get_adapters(&self) -> &Vec<wgpu::Adapter>;
    fn get_devices(&self) -> &Vec<wgpu::Device>;
    fn get_queues(&self) -> &Vec<wgpu::Queue>;
    fn get_available_deviceid(&self) -> Option<usize>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Runtime {
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
            flags: wgpu::InstanceFlags::empty(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        let adapters = Self::emulate_supported_adapters(&instance);
        
        let (devices, queues) = block_on(async {
            let mut devices = Vec::new();
            let mut queues = Vec::new();
            
            for adapter in &adapters {
                if let Ok((device, queue)) = adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                    },
                    None,
                ).await {
                    devices.push(device);
                    queues.push(queue);
                }
            }
            
            (devices, queues)
        });

        Runtime {
            instance,
            adapters,
            devices,
            queues,
        }
    }

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

    fn get_available_deviceid(&self) -> Option<usize> {

        for (i, _adapter) in self.adapters.iter().enumerate() {
            let device = &self.devices[i];
            device.poll(wgpu::Maintain::Wait);
            return Some(i);
        }
        None
        
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime struct")
    }
}

impl Runtime {

    fn emulate_supported_adapters(instance: &wgpu::Instance) -> Vec<wgpu::Adapter> {
        instance.enumerate_adapters(wgpu::Backends::all())
    }
}