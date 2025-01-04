use burn::backend::wgpu::{Wgpu, WgpuDevice};
use burn::tensor::Tensor;
use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::activation::relu;  // Changed import path

use circlenetic_wgpu_rs::{Runtime, WgpuRuntime};

type MyBackend = Wgpu;

#[derive(Module, Debug, Clone)]
struct SimpleNet {
    layer1: Linear<MyBackend>,
    layer2: Linear<MyBackend>,
}

impl SimpleNet {
    pub fn new(device: &WgpuDevice) -> Self {
        Self {
            layer1: LinearConfig::new(784, 128).init(device),
            layer2: LinearConfig::new(128, 10).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<MyBackend, 2>) -> Tensor<MyBackend, 2> {
        let x = self.layer1.forward(x);
        let x = relu(x);  // Changed from ReLU::new().forward(x)
        self.layer2.forward(x)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Initialize the runtime
    println!("Initializing WebGPU runtime...");
    let runtime: Runtime = Runtime::new();

    // Get the available adapters
    let adapters = runtime.get_adapters();
    println!("Available adapters: {:?}", adapters);

    // Get the available devices
    let devices = runtime.get_devices();
    println!("Available devices: {:?}", devices);

    // Get the available queues
    let queues = runtime.get_queues();
    println!("Available queues: {:?}", queues);

    // Obtain the available WgpuDevice
    println!("Obtaining available WgpuDevice...");

    //TODO: change to use runtime to get available WgpuDevice.from(device)
    // let device = WgpuDevice::default();
    let device_id = match runtime.get_available_deviceid() {
        Some(index) => index,
        None => return Err("No available device found".into()),
    };
    println!("Device index: {}", device_id);

    let wgpu_device = WgpuDevice::DiscreteGpu(device_id);
    
    println!("Creating model...");
    let model = SimpleNet::new(&wgpu_device);

    // Create a sample input tensor
    println!("Creating input tensor...");
    let input: Tensor<MyBackend, 2> = Tensor::random(
        [1, 784],
        burn::tensor::Distribution::Normal(0.0, 1.0),
        &wgpu_device,
    );
    println!("Input shape: {:?}", input.shape());

    println!("Input data: {:?}", input.clone().into_data().to_vec::<f32>().unwrap());
    
    println!("Running forward pass...");
    let output = model.forward(input);

    // Print results
    println!("Output shape: {:?}", output.shape());
    let output_data = output.into_data();
    let output_vec: Vec<f32> = output_data.to_vec().unwrap();
    println!("Output data: {:?}", output_vec);
    Ok(())
}