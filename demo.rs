// Generated Linea Rust code
use std::io::Write;
fn main() {
    let mut title : String = "=== Linea Programming Language ===".to_string();
    println!("{}", title);
    println!("{}", "".to_string());
    let mut a : i64 = 100;
    let mut b : i64 = 25;
    println!("{}", "Arithmetic Demo:".to_string());
    println!("{}", format!("{}{}", "a = ".to_string(), ((a).to_string())));
    println!("{}", format!("{}{}", "b = ".to_string(), ((b).to_string())));
    println!("{}", format!("{}{}", "a + b = ".to_string(), (((a + b)).to_string())));
    println!("{}", format!("{}{}", "a * b = ".to_string(), (((a * b)).to_string())));
    println!("{}", format!("{}{}", "a / b = ".to_string(), (((a / b)).to_string())));
    println!("{}", "".to_string());
    println!("{}", "Loop Demo (0 to 5):".to_string());
    for i in 0..=5 {
        println!("{}", format!("{}{}", "  i = ".to_string(), ((i).to_string())));
    }
    println!("{}", "".to_string());
    let mut str_num : String = "42".to_string();
    (str_num.parse::<i64>().unwrap_or(0));
    println!("{}", "Type Casting Demo:".to_string());
    println!("{}", format!("{}{}", "String '42' + 8 = ".to_string(), format!("{}{}", str_num, ((8).to_string()))));
    println!("{}", "".to_string());
    println!("{}", "✅ Demo complete!".to_string());
}
