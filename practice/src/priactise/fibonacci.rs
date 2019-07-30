use std::io;
fn fibonacci(n: u32) -> u64 {
    let mut array: [u64; 2] = [0, 1];
    match n {
        0 => array[0],
        1 => array[1],
        _ => {
            let mut current = 2;
            while current < n {
                current += 1;
                let temp = array[0] + array[1];
                array[0] = array[1];
                array[1] = temp;
            }
            array[0] + array[1]
        }
    }
}

pub fn print_fibonacci (){
    println!("fibonacci: input a number");
    let mut num = String::new();
    loop {
        io::stdin()
            .read_line(&mut num)
            .expect("Failled to read line");
        let n: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!(" fibonacci {} = {}", n, fibonacci(n));
        num.clear();
    }
}