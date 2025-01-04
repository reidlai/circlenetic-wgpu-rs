use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // If no arguments, show help
    if args.len() <= 1 {
        print_help();
        return;
    }

    // Check for --example=nn-example format
    if let Some(arg) = args.get(1) {
        if arg.starts_with("--example=") {
            let example = arg.trim_start_matches("--example=");
            run_example(example);
            return;
        }
    }

    // If arguments exist but not in correct format, show help
    print_help();
}

fn print_help() {
    println!("WGPU examples and utilities");
    println!("\nUsage: cargo run --example=<name>");
    println!("\nAvailable examples:");
    println!("  --example=nn-example    Run the neural network example");
}

fn run_example(name: &str) {
    match name {
        "nn-example" => {
            // Run the example as a separate process
            let status = Command::new("cargo")
                .args(["run", "--example", "nn-example"])
                .status()
                .expect("Failed to execute example");

            if !status.success() {
                eprintln!("Example failed to run");
                std::process::exit(1);
            }
        }
        _ => {
            println!("Unknown example: {}", name);
            print_help();
        }
    }
}
