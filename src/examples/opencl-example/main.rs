use burn::backend::opencl::OpenCL;
use burn::tensor::Tensor;

fn main() {
    println!("Running OpenCL example...");
    
    // Initialize OpenCL backend
    let backend = OpenCL::default();
    
    // Create some example tensors
    let a: Tensor<OpenCL, 2> = Tensor::from_data([[1.0, 2.0], [3.0, 4.0]]);
    let b: Tensor<OpenCL, 2> = Tensor::from_data([[5.0, 6.0], [7.0, 8.0]]);
    
    // Perform matrix multiplication
    let c = a.matmul(b);
    
    println!("Matrix A:");
    println!("{:?}", a);
    println!("\nMatrix B:");
    println!("{:?}", b);
    println!("\nMatrix multiplication result (A × B):");
    println!("{:?}", c);
}
