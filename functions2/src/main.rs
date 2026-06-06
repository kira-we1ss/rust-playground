fn is_adult(x: u32) -> bool {
    if x >= 18 { true } else { false }
}

fn main() {
    let x = 19;
    let y = 17;
    let result = is_adult(x);
    let result2 = is_adult(y);
    println!("{result}, {result2}");
}
