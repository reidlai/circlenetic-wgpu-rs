use burn::backend::wgpu::{Wgpu, WgpuDevice};
use burn::tensor::Tensor;
use burn::module::Module;
use burn::nn::{Linear, LinearConfig};
use burn::tensor::activation::relu;  // Changed import path

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
    // Initialize the WebGPU backend
    println!("Initializing WebGPU device...");
    let device = WgpuDevice::default();
    
    println!("Creating model...");
    let model = SimpleNet::new(&device);

    // Create a sample input tensor
    println!("Creating input tensor...");
    let input: Tensor<MyBackend, 2> = Tensor::random(
        [1, 784],
        burn::tensor::Distribution::Normal(0.0, 1.0),
        &device,
    );

    println!("Input shape: {:?}", input.shape());
    
    println!("Running forward pass...");
    let output = model.forward(input);

    // Print results
    println!("Output shape: {:?}", output.shape());
    let output_data = output.into_data();
    let output_vec: Vec<f32> = output_data.to_vec().unwrap();
    println!("Output data: {:?}", output_vec);
    Ok(())
}