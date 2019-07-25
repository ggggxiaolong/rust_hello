fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count:1,
    };
    println!("{}", user1.email);
    let user2 = user_builder(String::from("jayden"), String::from("jayden.lee@gmail.com"));
    println!("{}", user2.username);

    let user3 = User{
        email:String::from("wan@hallo.com"),
        username:String::from("fanchengcheng"),
        ..user2
    };

    println!("{}, {}", user3.active, user3.sign_in_count);
    let black = Color(0,0,0);
    test_rectangle();
    test_rec_hold();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);

fn user_builder(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count:1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    // 无参数
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 有参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.height > other.height && self.width > other.width) ||
            (self.width > other.height && self.height > other.width)
    }

    // 构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }
}

fn test_rectangle(){
    let r = Rectangle{width:20, height:34};
    println!("{:#?} area is {}",r, r.area());
}

fn test_rec_hold(){
    let r1 = Rectangle{
        width:30,
        height:50,
    };
    let r2 = Rectangle{
        width: 10,
        height: 40,
    };
    let r3 = Rectangle{
        width: 60,
        height: 45,
    };
    for r in [r2, r3].iter() {
        println!("Can r1 hold {}", r1.can_hold(r));
    }
}