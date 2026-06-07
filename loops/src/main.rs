fn simpleloop(mut x: u32) {
    loop {
        if x <= 10 {
            println!("{x}");
            x += 1;
        } else {
            break;
        }
    }
}

fn whileloop(mut x: u32) {
    while x <= 10 {
        println!("{x}");
        x += 1;
    }
}

fn forloop() {
    for x in 1..=10 {
        println!("{x}");
    }
}

fn main() {
    let x = 1;
    println!("--- simple ---");
    simpleloop(x);
    println!("--- while ---");
    whileloop(x);
    println!("--- for ---");
    forloop();
}
