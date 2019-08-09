fn main() {
    println!("{}", do_twice(|x| x + 1, 2));
    println!("{}", do_twice(add_one, 5));
    let a = vec![1,2];
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

mod hello_macro {
    pub trait HelloMacro {
        fn hello_macro();
    }

    
}