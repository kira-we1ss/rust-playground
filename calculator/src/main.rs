fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn calculate(a: f64, b: f64, op: char) -> f64 {
    if op == '+' {
        let result = add(a, b);
        return result;
    } else if op == '-' {
        let result = subtract(a, b);
        return result;
    } else if op == '*' {
        let result = multiply(a, b);
        return result;
    } else if op == '/' {
        if b == 0.0 {
            return 0.0;
        } else {
            let result = divide(a, b);
            return result;
        }
    } else {
        return 0.0;
    }
}

fn main() {
    let x = 8.0;
    let y = 0.0;
    let result = calculate(x, y, '/');
    println!("{result}");
}
