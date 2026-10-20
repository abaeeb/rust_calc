use std::{io::{self, Write}};
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