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
                    if let Err(e) = fs::write(&rs_file, rust_code) {
                        eprintln!("✗ Error writing Rust file: {}", e);
                        std::process::exit(1);
                    }

                    match Command::new("rustc")
                        .arg("-O")
                        .arg(&rs_file)
                        .arg("-o")
                        .arg(&output_path)
                        .output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✓ Compilation successful!");
                                println!("Output: {}", output_path.display());
                                let _ = fs::remove_file(&rs_file);
                            } else {
                                eprintln!("✗ Rustc compilation failed");
                                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to run rustc: {}", e);
                            eprintln!("Make sure Rust compiler is installed: rustup install stable");
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
