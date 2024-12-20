use circlenetic_wgpu_rs::{Runtime, WgpuRuntime};
fn main() {
    let runtime = Runtime::new();
    println!("Runtime version: {}", runtime);
    println!("{:?}", runtime.get_adapters());

    let devices = runtime.get_devices();

    // Example workload distribution
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Example data
    println!("i: {}", devices.len());
    let chunk_size = data.len() / devices.len();
    data.chunks(chunk_size).enumerate().for_each(|(i, chunk)| {
        println!("i: {:?}, chunk: {:?}", i, chunk);
        let device = &devices[i];

        // Perform operations on the chunk using the device and queue
        println!("Processing chunk {:?} on device {:?}", chunk, device);
    });
}