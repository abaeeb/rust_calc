use std::io::{self, Write};
fn add(a: f64, b: f64) -> f64 {a + b}
fn sub(a: f64, b: f64) -> f64 {a - b}
fn mul(a: f64, b: f64) -> f64 {a * b}
fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("На ноль делить нельзя".to_string())
    } else {
        Ok(a / b)
    }
}
fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(add(a,b)),
        "-" => Ok(sub(a,b)),
        "*" => Ok(mul(a,b)),
        "/" => div(a, b),
        _=>Err("Неизвестная операция".to_string()),
        
    }
}
fn main() {
    println!("Калькулятор");
    println!("Для выхода введите 'q' в поле первого числа.");
    loop {
        print!("\nПервое число: ");
        io::stdout().flush().unwrap();
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).unwrap();
        let input1 = input1.trim();
        if input1.eq_ignore_ascii_case("q") {
            println!("Выход");
            break;
        }
    }
    
}