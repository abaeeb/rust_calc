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
