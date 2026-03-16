use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use linea_ast::parse;
use linea_executor::Executor;
use linea_codegen::generate_rust_code;

#[derive(ClapParser)]
#[command(name = "linea")]
#[command(about = "The Linea Programming Language Compiler", long_about = None)]
#[command(version = "3.2.0-alpha-1")]
#[command(author = "Gautham Nair <https://github.com/gauthamnair2005>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Linea source file to an executable
    Compile {
        /// Input Linea source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output executable path
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Run a Linea source file directly (interpreted)
    Run {
        /// Input Linea source file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Parse a Linea file and display the AST
    Parse {
        /// Input Linea source file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Generate Rust code from Linea source
    GenRust {
        /// Input Linea source file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output Rust file
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

fn compile_file(input: &PathBuf, output: Option<PathBuf>) {
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
                        p.set_extension("");
                        p
                    });

                    let rs_file = format!("{}.rs", output_path.display());
                    // Create a temporary Cargo project
                    let project_name = output_path.file_stem().unwrap().to_str().unwrap();
                    let build_dir = std::env::temp_dir().join(format!("linea_build_{}", project_name));
                    
                    if build_dir.exists() {
                        let _ = fs::remove_dir_all(&build_dir);
                    }
                    fs::create_dir_all(build_dir.join("src")).expect("Failed to create build dir");
                    
                    // Write Cargo.toml
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
"#, project_name);

                    fs::write(build_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");
                    fs::write(build_dir.join("src/main.rs"), rust_code).expect("Failed to write main.rs");

                    println!("Building with Cargo in {}...", build_dir.display());

                    match Command::new("cargo")
                        .arg("build")
                        .arg("--release")
                        .current_dir(&build_dir)
                        .output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✓ Compilation successful!");
                                
                                // Move binary to output location
                                let binary_name = if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.to_string() };
                                let built_binary = build_dir.join("target/release").join(&binary_name);
                                let final_output = if output_path.extension().is_none() && cfg!(windows) {
                                    output_path.with_extension("exe")
                                } else {
                                    output_path.clone()
                                };
                                
                                match fs::copy(&built_binary, &final_output) {
                                    Ok(_) => println!("Output: {}", final_output.display()),
                                    Err(e) => eprintln!("✗ Error copying binary: {}", e),
                                }
                                
                                // Cleanup
                                // let _ = fs::remove_dir_all(&build_dir);
                            } else {
                                eprintln!("✗ Cargo compilation failed");
                                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
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
            eprintln!("✗ Compilation failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_file(input: &PathBuf) {
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
                    // Program executed successfully, output already printed
                }
                Err(e) => {
                    eprintln!("✗ Runtime error: {}", e);
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

fn parse_file(input: &PathBuf) {
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
                        Ok(_) => println!("✓ Generated Rust code: {}", output_path.display()),
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
