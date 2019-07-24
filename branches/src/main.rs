fn main() {
    let x = 3;
    if x < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false")
    }

    let x = if true { 6 } else { 5 };
    println!("The value of x is {}", x);
    for_in();
}

fn for_in() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!")
}