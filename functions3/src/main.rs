fn is_positive(x: i32) -> bool {
    if x < 0 {
        return false;
    } else {
        return true;
    }
}

fn multiply(x: i32, y: i32) -> i32 {
    let product = x * y;
    return product;
}

fn multiply_if_positive(x: i32, y: i32) -> i32 {
    let positive_x = is_positive(x);
    let positive_y = is_positive(y);
    let result = multiply(x, y);
    if positive_x == true {
        if positive_y == true {
            return result;
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

fn main() {
    let x = 5;
    let y = 6;
    let result1 = is_positive(x);
    let result2 = is_positive(y);
    let result3 = multiply(x, y);
    let result4 = multiply_if_positive(x, y);
    println!("{result1}, {result2}, {result3}, {result4}")
}
