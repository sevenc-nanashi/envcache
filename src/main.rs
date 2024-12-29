use sha2::{Digest, Sha256};
use std::io::Write;

fn generate_key(args: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(env!("CARGO_PKG_VERSION"));
    hasher.update("\n");
    for arg in args {
        hasher.update(arg);
        hasher.update("\n");
    }
    let result = hasher.finalize();
    format!("{:x}", result)
}
fn print_help() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("Usage: {} <command> [args...]", args[0]);
    println!("Options:");
    println!("  --version    Print version information");
    println!("  --help       Print this help message");
    println!("  --cache-dir  Print the cache directory");
    println!("  --purge      Purge the cache directory");
    println!();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        print_help();
        std::process::exit(1);
    }
    if args[1].starts_with("--") {
        match args[1].as_str() {
            "--cache-dir" => {
                println!(
                    "{}",
                    dirs::cache_dir()
                        .unwrap()
                        .join(env!("CARGO_PKG_NAME"))
                        .display()
                );
                std::process::exit(0);
            }
            "--purge" => {
                let path = dirs::cache_dir().unwrap().join(env!("CARGO_PKG_NAME"));
                if path.exists() {
                    std::fs::remove_dir_all(&path).unwrap();
                }
                println!("Purged cache directory: {}", path.display());
                std::process::exit(0);
            }
            "--version" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                println!("Unknown option: {}", args[0]);
                std::process::exit(1);
            }
        }
    }

    let key = generate_key(&args[1..]);
    let path = dirs::cache_dir()
        .unwrap()
        .join(env!("CARGO_PKG_NAME"))
        .join(key);
    if !path.exists() {
        let process = std::process::Command::new(&args[1])
            .args(&args[2..])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap();
        let output = process.wait_with_output().unwrap();
        if !output.status.success() {
            std::process::exit(output.status.code().unwrap());
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &output.stdout).unwrap();
        std::io::stdout().write_all(&output.stdout).unwrap();
    } else {
        let output = std::fs::read(&path).unwrap();
        std::io::stdout().write_all(&output).unwrap();
    }
}
