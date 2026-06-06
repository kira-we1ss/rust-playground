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
        add(a, b)
    } else if op == '-' {
        subtract(a, b)
    } else if op == '*' {
        multiply(a, b)
    } else if op == '/' {
        if b == 0.0 {
            return 0.0;
        } else {
            divide(a, b)
        }
    } else {
        return 0.0;
    }
}

fn main() {
    let x = 8.0;
    let y = 0.0;
    let op = '/';
    let result = calculate(x, y, op);
    println!("{result}");
}
