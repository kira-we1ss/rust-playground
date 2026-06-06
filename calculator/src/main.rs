fn calculate(a: f64, b: f64, op: char) -> f64 {
    if op == '+' {
        a + b
    } else if op == '-' {
        a - b
    } else if op == '*' {
        a * b
    } else if op == '/' {
        if b == 0.0 {
            return 0.0;
        } else {
            a / b
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
