use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use linea_ast::parse;
use linea_executor::Executor;
use linea_codegen::generate_rust_code;

fn detected_parallel_jobs() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

#[derive(Parser)]
#[command(name = "linea")]
#[command(about = "Linea Compiler | High-Performance AI & Data Language", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "Gautham Nair")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile Linea source to optimized native executable
    Compile {
        /// Input source file (.ln)
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output executable path
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Run Linea source directly (Interpreter Mode)
    Run {
        /// Input source file (.ln)
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Debug: Inspect Abstract Syntax Tree (AST)
    Parse {
        /// Input source file (.ln)
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Debug: Generate intermediate Rust code
    GenRust {
        /// Input source file (.ln)
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output Rust file (.rs)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output } => {
            compile_file(&input, output);
        }
        Commands::Run { input } => {
            run_file(&input);
        }
        Commands::Parse { input } => {
            parse_file(&input);
        }
        Commands::GenRust { input, output } => {
            gen_rust_file(&input, output);
        }
    }
}

fn print_header(action: &str, file: &std::path::Path) {
    println!(
        "Linea Compiler v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Action: {} | Target: {}", action, file.display());
    println!("--------------------------------------------------");
}

fn compile_file(input: &PathBuf, output: Option<PathBuf>) {
    print_header("Compiling", input);
    
    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    println!(">> Parsing source code...");
    match parse(&source) {
        Ok(program) => {
            println!(">> Generating native backend code...");
            match generate_rust_code(&program) {
                Ok(rust_code) => {
                    let output_path = output.clone().unwrap_or_else(|| {
                        let mut p = input.clone();
                        p.set_extension("");
                        p
                    });
                    
                    // Normalize project name (remove extension, ensure valid crate name)
                    let file_stem = input.file_stem().unwrap().to_str().unwrap();
                    let project_name = file_stem.replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

                    let build_dir = std::env::temp_dir().join(format!("linea_build_{}", project_name));
                    
                    if build_dir.exists() {
                        let _ = fs::remove_dir_all(&build_dir);
                    }
                    fs::create_dir_all(build_dir.join("src")).expect("Failed to create build dir");
                    
                    // Write Cargo.toml with dependencies
                    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
csv = "1.3"
rand = "0.8"
reqwest = {{ version = "0.12", features = ["blocking", "json", "rustls-tls"], default-features = false }}
serde_json = "1.0"
comrak = "0.18"
calamine = "0.24"
rust_xlsxwriter = "0.68"
plotters = {{ version = "0.3.7", default-features = false, features = ["bitmap_backend", "bitmap_encoder", "line_series", "point_series", "histogram", "ab_glyph"] }}
wgpu = "0.20"
pollster = "0.3"
bytemuck = {{ version = "1.14", features = ["derive"] }}
futures = "0.3"
iced = "0.13"
"#, project_name);

                    fs::write(build_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");
                    fs::write(build_dir.join("src/main.rs"), rust_code).expect("Failed to write main.rs");

                    let parallel_jobs = detected_parallel_jobs();
                    println!(
                        ">> Building optimized binary (release mode) using {} parallel jobs...",
                        parallel_jobs
                    );

                    let mut cargo_cmd = Command::new("cargo");
                    cargo_cmd
                        .arg("build")
                        .arg("--release")
                        .arg("--jobs")
                        .arg(parallel_jobs.to_string())
                        .current_dir(&build_dir);

                    match cargo_cmd.output() {
                        Ok(cargo_output) => {
                            if cargo_output.status.success() {
                                println!(">> Finalizing build...");
                                
                                // Move binary to output location
                                let binary_name = if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.to_string() };
                                let built_binary = build_dir.join("target/release").join(&binary_name);
                                
                                let final_output_path = if output.is_some() {
                                    output.clone().unwrap()
                                } else {
                                    // Default output is current_dir/filename (no extension on linux, .exe on windows)
                                    let mut p = PathBuf::from(".");
                                    p.push(if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.to_string() });
                                    p
                                };

                                match fs::copy(&built_binary, &final_output_path) {
                                    Ok(_) => {
                                        println!("--------------------------------------------------");
                                        println!("✓ SUCCESS: Build complete.");
                                        println!("  Artifact: {}", final_output_path.display());
                                    },
                                    Err(e) => eprintln!("✗ Error copying binary: {}", e),
                                }
                            } else {
                                eprintln!("--------------------------------------------------");
                                eprintln!("✗ FAILURE: Compilation failed.");
                                eprintln!("Details:\n{}", String::from_utf8_lossy(&cargo_output.stderr));
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to run cargo: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Code generation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(input: &PathBuf) {
    print_header("Executing (Interpreter)", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    match parse(&source) {
        Ok(program) => {
            let mut executor = Executor::new();
            match executor.execute(&program) {
                Ok(_) => {
                    println!("--------------------------------------------------");
                    println!("✓ Execution finished successfully.");
                }
                Err(e) => {
                    eprintln!("--------------------------------------------------");
                    eprintln!("✗ Runtime Exception: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("--------------------------------------------------");
            eprintln!("✗ Syntax Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_file(input: &PathBuf) {
    print_header("Debugging AST", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    match parse(&source) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn gen_rust_file(input: &PathBuf, output: Option<PathBuf>) {
    print_header("Generating Intermediate Rust", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    match parse(&source) {
        Ok(program) => {
            match generate_rust_code(&program) {
                Ok(rust_code) => {
                    let output_path = output.unwrap_or_else(|| {
                        let mut p = input.clone();
                        p.set_extension("rs");
                        p
                    });

                    match fs::write(&output_path, rust_code) {
                        Ok(_) => {
                            println!("✓ Generated Rust source: {}", output_path.display());
                        },
                        Err(e) => {
                            eprintln!("✗ Error writing file: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Code generation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
