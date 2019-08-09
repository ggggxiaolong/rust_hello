use gui::{Button, Draw, Screen};
use blog::{Post};

fn main() {
    test_post();
}

fn test_post(){
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());
}

fn _test_screen(){
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };
    screen.run();
}

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self){
        for option in self.options.iter() {
            println!("Draw SelectBox option: {}", option);
        }
    }
}

mod gui {
    pub trait Draw {
        fn draw(&self) {}
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self){
            println!("Draw button {}", self.label);
        }
    }

    pub struct TextFiled {
        pub width: u32,
        pub height: u32,
        pub label: String,
        pub placeholder: String,
    }

    impl Draw for TextFiled {
        fn draw(&self){
            println!("Draw TextFiled, label {}, placeholder {}", self.label, self.placeholder);
        }
    }
}

mod blog{

    enum State{

    }
    pub struct Post{
        content: String,
    }

    pub struct DrafPost{
        content: String
    }

    impl DrafPost {
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost{
                content: self.content
            }
        }

        pub fn add_text(&mut self, content: &str){
            self.content.push_str(content);
        }
    }

    pub struct PendingReviewPost{
        content: String
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post{
            Post{
                content:self.content
            }
        }
    }

    impl Post {
        pub fn content(&self) -> &str {
            &self.content
        }

        pub fn new() -> DrafPost {
            DrafPost{
                content:String::new(),
            }
        }
    }
}