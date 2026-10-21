use std::io::{self, Write};

fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }

fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("На ноль делить нельзя".to_string())
    } else {
        Ok(a / b)
    }
}

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(add(a, b)),
        "-" => Ok(sub(a, b)),
        "*" => Ok(mul(a, b)),
        "/" => div(a, b),
        _ => Err("Неизвестная операция".to_string()),
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
        
        let a: f64 = match input1.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка: введите корректное число!");
                continue;
            }
        };

        print!("Выберите операцию (+,-,*,/): ");
        io::stdout().flush().unwrap();
        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        let op = op.trim();

        print!("Второе число: ");
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        
        let b: f64 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ошибка: введите корректное число!");
                continue;
            }
        };

        match calculate(a, op, b) {
            Ok(result) => println!("Результат: {}", result),
            Err(err_msg) => println!("Ошибка: {}", err_msg),
        }
    } 
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_operations() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(sub(5.0, 2.0), 3.0);
        assert_eq!(mul(4.0, 2.5), 10.0);
        assert_eq!(div(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(div(10.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate_function() {
        assert_eq!(calculate(2.0, "+", 2.0).unwrap(), 4.0);
        assert!(calculate(2.0, "%", 2.0).is_err());
    }
}