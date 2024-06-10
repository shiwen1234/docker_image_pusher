use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "docker-simplifier",
    version = "1.0",
    about = "Simplifies Docker pull commands"
)]
struct Cli {
    #[arg(short, long)]
    image: String,
}

fn main() {
    let args = Cli::parse();

    // Define the base URL for the Alibaba Cloud Docker registry
    let base_url = "registry.cn-hangzhou.aliyuncs.com/docker-hub-images/";

    // Construct the full Docker pull command
    let full_command = format!("{}{}", base_url, args.image);

    // Execute the Docker pull command
    let output = Command::new("docker")
        .arg("pull")
        .arg(&full_command)
        .output()
        .expect("Failed to execute Docker command");

    // Print the output of the Docker pull command
    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
}
