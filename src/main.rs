use std::env;

fn add(num1 : f64, num2 : f64) -> f64 {
    num1 + num2
}

fn subtract(num1 : f64, num2 : f64) -> f64 {
    num1 - num2
}

fn multiply(num1 : f64, num2 : f64) -> f64 {
    num1 * num2
}

fn divide(num1 : f64, num2 : f64) -> f64 {
    if num2 == 0f64 {
        eprintln!("Error: Division by zero");
        std::process::exit(1)
    }
    num1 / num2
}

fn remainder(num1 : i32, num2 : i32) -> i32 {
    num1 % num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1.0, 3.0), 4.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10.5, 3.0), 7.5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4.0, 3.0), 12.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(15.0, 3.0), 5.0);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(15.0, 0.0);
    }

    #[test]
    fn test_remainder() {
        assert_eq!(remainder(9, 4), 1);
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("How to use it:\n");
        eprintln!("simpleCalculator <num1> <operator> <num2>");
        eprintln!("Operator can be +, -, *, /, %");
        std::process::exit(1);
    }

    let n1 : f64 = args[1].parse().expect("Invalid Number");
    let operator = &args[2];
    let n2 : f64 = args[3].parse().expect("Invalid Number");

    let result = match operator.as_str() {
        "+" => add(n1, n2),
        "-" => subtract(n1, n2),
        "/" => divide(n1, n2),
        "*" => multiply(n1, n2),
        "%" => remainder(n1 as i32, n2 as i32) as f64,
        _ => {
            eprintln!("Unsupported Operator! {}", operator);
            std::process::exit(1);
        }
    };

    println!("{} {} {} = {}", n1, operator, n2, result);
}
