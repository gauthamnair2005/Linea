use std::fs;
use std::path::PathBuf;
use std::process::Command;
use linea_ast::parse;
use linea_executor::Executor;
use linea_codegen::generate_rust_code;

const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_RESET: &str = "\x1b[0m";

fn success_msg(message: &str) -> String {
    format!("{ANSI_BOLD}{ANSI_GREEN}{message}{ANSI_RESET}")
}

fn error_msg(message: &str) -> String {
    format!("{ANSI_BOLD}{ANSI_RED}{message}{ANSI_RESET}")
}

fn detected_parallel_jobs() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

enum Commands {
    Compile {
        input: PathBuf,
        output: Option<PathBuf>,
    },
    Run {
        input: PathBuf,
    },
    Parse {
        input: PathBuf,
    },
    GenRust {
        input: PathBuf,
        output: Option<PathBuf>,
    },
}

fn print_usage() {
    println!("Linea Compiler v{}", env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("  linea compile <file.ln> [-o <output>]");
    println!("  linea run <file.ln>");
    println!("  linea parse <file.ln>");
    println!("  linea gen-rust <file.ln> [-o <output.rs>]");
    println!("  linea --version");
}

fn parse_cli() -> Result<Commands, String> {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else {
        print_usage();
        return Err("missing command".to_string());
    };

    if cmd == "-h" || cmd == "--help" {
        print_usage();
        return Err("help".to_string());
    }
    if cmd == "-V" || cmd == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Err("help".to_string());
    }

    let parse_output_flag = |tail: Vec<String>| -> Result<Option<PathBuf>, String> {
        if tail.is_empty() {
            return Ok(None);
        }
        if tail.len() == 2 && (tail[0] == "-o" || tail[0] == "--output") {
            return Ok(Some(PathBuf::from(&tail[1])));
        }
        Err("invalid output flag usage. expected: -o <path>".to_string())
    };

    match cmd.as_str() {
        "compile" => {
            let Some(input) = args.next() else {
                return Err("compile requires an input .ln file".to_string());
            };
            let tail: Vec<String> = args.collect();
            let output = parse_output_flag(tail)?;
            Ok(Commands::Compile { input: PathBuf::from(input), output })
        }
        "run" => {
            let Some(input) = args.next() else {
                return Err("run requires an input .ln file".to_string());
            };
            Ok(Commands::Run { input: PathBuf::from(input) })
        }
        "parse" => {
            let Some(input) = args.next() else {
                return Err("parse requires an input .ln file".to_string());
            };
            Ok(Commands::Parse { input: PathBuf::from(input) })
        }
        "gen-rust" => {
            let Some(input) = args.next() else {
                return Err("gen-rust requires an input .ln file".to_string());
            };
            let tail: Vec<String> = args.collect();
            let output = parse_output_flag(tail)?;
            Ok(Commands::GenRust { input: PathBuf::from(input), output })
        }
        _ => Err(format!("unknown command '{}'", cmd)),
    }
}

fn main() {
    let command = match parse_cli() {
        Ok(cmd) => cmd,
        Err(e) if e == "help" => return,
        Err(e) => {
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
            print_usage();
            std::process::exit(1);
        }
    };

    match command {
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
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
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

                    let build_dir = std::env::temp_dir()
                        .join("linea_build_cache")
                        .join(&project_name);
                    let target_cache_dir = std::env::temp_dir().join("linea_target_cache");

                    fs::create_dir_all(build_dir.join("src")).expect("Failed to create build dir");
                    fs::create_dir_all(&target_cache_dir).expect("Failed to create shared target cache");
                    
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
calamine = "0.24"
rust_xlsxwriter = "0.68"
rusqlite = {{ version = "0.31.0", features = ["bundled"] }}
sha2 = "0.10"
md5 = "0.7"
plotters = {{ version = "0.3.7", default-features = false, features = ["bitmap_backend", "bitmap_encoder", "line_series", "point_series", "histogram", "ab_glyph"] }}
wgpu = "0.20"
pollster = "0.3"
bytemuck = {{ version = "1.14", features = ["derive"] }}
futures = "0.3"
tiny_http = "0.12"
"#, project_name);

                    fs::write(build_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");
                    fs::write(build_dir.join("src/main.rs"), rust_code).expect("Failed to write main.rs");

                    let parallel_jobs = detected_parallel_jobs();
                    println!(
                        ">> Building optimized binary (release mode) using {} parallel jobs...",
                        parallel_jobs
                    );
                    println!(">> Using shared build cache: {}", target_cache_dir.display());

                    let mut cargo_cmd = Command::new("cargo");
                    cargo_cmd
                        .arg("build")
                        .arg("--release")
                        .arg("--jobs")
                        .arg(parallel_jobs.to_string())
                        .env("CARGO_TARGET_DIR", &target_cache_dir)
                        .current_dir(&build_dir);

                    match cargo_cmd.output() {
                        Ok(cargo_output) => {
                            if cargo_output.status.success() {
                                println!(">> Finalizing build...");
                                
                                // Move binary to output location
                                let binary_name = if cfg!(windows) { format!("{}.exe", project_name) } else { project_name.to_string() };
                                let built_binary = target_cache_dir.join("release").join(&binary_name);
                                
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
                                        println!("{}", success_msg("✓ SUCCESS: Build complete."));
                                        println!("  Artifact: {}", final_output_path.display());
                                    },
                                    Err(e) => eprintln!("{} Error copying binary: {}", error_msg("✗ FAILURE:"), e),
                                }
                            } else {
                                eprintln!("--------------------------------------------------");
                                eprintln!("{}", error_msg("✗ FAILURE: Compilation failed."));
                                eprintln!("Details:\n{}", String::from_utf8_lossy(&cargo_output.stderr));
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("{} Failed to run cargo: {}", error_msg("✗ FAILURE:"), e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} Code generation failed: {}", error_msg("✗ FAILURE:"), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{} Parse error: {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    }
}

fn run_file(input: &PathBuf) {
    print_header("Executing (Interpreter)", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    };

    match parse(&source) {
        Ok(program) => {
            let mut executor = Executor::new();
            match executor.execute(&program) {
                Ok(_) => {
                    println!("--------------------------------------------------");
                    println!("{}", success_msg("✓ SUCCESS: Execution finished successfully."));
                }
                Err(e) => {
                    eprintln!("--------------------------------------------------");
                    eprintln!("{} Runtime Exception: {}", error_msg("✗ FAILURE:"), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("--------------------------------------------------");
            eprintln!("{} Syntax Error: {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    }
}

fn parse_file(input: &PathBuf) {
    print_header("Debugging AST", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    };

    match parse(&source) {
        Ok(program) => {
            println!("{:#?}", program);
        }
        Err(e) => {
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    }
}

fn gen_rust_file(input: &PathBuf, output: Option<PathBuf>) {
    print_header("Generating Intermediate Rust", input);

    let source = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} {}", error_msg("✗ FAILURE:"), e);
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
                            println!("{} Generated Rust source: {}", success_msg("✓ SUCCESS:"), output_path.display());
                        },
                        Err(e) => {
                            eprintln!("{} Error writing file: {}", error_msg("✗ FAILURE:"), e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} Code generation failed: {}", error_msg("✗ FAILURE:"), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{} Parse error: {}", error_msg("✗ FAILURE:"), e);
            std::process::exit(1);
        }
    }
}
