use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3];
    v.get(0);
    let mut v = Vec::new();
    v.push(1);

    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // for i in &v {
    //     println!("{}", i);
    // }

    let mut v = v;
    for i in &mut v {
        *i += 3;
    }
    // print_vec(&v);

    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 字符串格式化
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let scorse: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn print_vec(v : &Vec<i32>){
    for i in v {
        println!("{}", i);
    }
}
