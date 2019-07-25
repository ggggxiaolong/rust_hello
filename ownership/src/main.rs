mod slice;
fn main() {
    // let s = gives_ownership();
    // let s2 = take_and_give_back(String::from("world"));
    // println!("{}, {}", s, s2);
    let s1 = String::from("hello");
    let len = caculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    slice::test_slice();
    let s = String::from("hello world");
    let word = slice::first_word(&s);
    println!("{}", word);
}

fn caculate_length(s: &String) ->usize {s.len()}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String{
//     String::from("hello")
// }

// fn take_and_give_back(some_string: String) -> String {some_string}
