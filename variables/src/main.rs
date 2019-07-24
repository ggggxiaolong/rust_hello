fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, _, z) = tup;
    println!("The value of x is {}", x);
    println!("The value of y is {}", tup.1);
    println!("The value of z is {}", z);
    let array: [i32; 5] = [1,2,3,4,5];
    println!("First num in array {}", array[0]);
    another_method(x, tup.1);
    // test method return
    println!("Method return {}", five())
}


fn another_method(x: i32, y:f64) {
    println!("The param of x is {}", x);
    println!("The param of y is {}", y);
}

fn five() -> i32 {
    5
}