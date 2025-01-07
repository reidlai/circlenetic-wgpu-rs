use burn::backend::ndarray::NdArray;
use burn::tensor::Tensor;
use burn::backend::ndarray::NdArrayDevice;

fn main() {
    println!("Running Ndarray (CPU) example...");
    
    // Initialize Ndarray backend and device
    let device = NdArrayDevice::default();
    
    // Create some example tensors
    let a: Tensor<NdArray, 2> = Tensor::from_data([[1.0f32, 2.0], [3.0, 4.0]], &device);
    let b: Tensor<NdArray, 2> = Tensor::from_data([[5.0f32, 6.0], [7.0, 8.0]], &device);
    
    // Perform matrix multiplication
    let c = a.clone().matmul(b.clone());
    
    println!("Matrix A:");
    println!("{:?}", a);
    println!("\nMatrix B:");
    println!("{:?}", b);
    println!("\nMatrix multiplication result (A × B):");
    println!("{:?}", c);
    
    // Additional operations using tensor methods
    println!("\nElement-wise addition (A + B):");
    println!("{:?}", a.clone().add(b.clone()));
    
    println!("\nElement-wise multiplication (A * B):");
    println!("{:?}", a.clone().mul(b.clone()));
    
    // Compute mean and sum
    println!("\nMean of matrix A along dimension 0:");
    println!("{:?}", a.clone().mean_dim(0));
    
    println!("\nSum of matrix A along dimension 0:");
    println!("{:?}", a.sum_dim(0));
}
