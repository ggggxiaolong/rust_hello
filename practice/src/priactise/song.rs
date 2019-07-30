
fn print_day(day: &str){
    println!("On the {} day of Christmas my true love sent to me ", day)
}

fn others(d: u8){
    if d == 1 {
        println!("A partridge in a pear tree");
        return;
    }
    let mut index = d as i32 + 1;
    while index >= 1 {
        match index{
            1 => println!("And a partridge in a pear tree"),
            2 => println!("Two turtle doves "),
            3 => println!("Three French hens"),
            4 => println!("Four calling birds"),
            5 => println!("Five gold rings"),
            6 => println!("Six geese a laying"),
            7 => println!("Seven swans a swimming"),
            8 => println!("Eight maids a milking"),
            9 => println!("Nine, drummers drumming"),
            10 => println!("Ten pipers piping"),
            11 => println!("Eleven ladies dancing"),
            _ => println!("Twelve Lords a leaping")
        }
        index -= 1;
    }
}
pub fn print_song(){
    let day_array = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleven", "twelfth"];
    let mut day: u8 = 0;
    while day < 12 {
        print_day(day_array[day as usize]);
        others(day);
        day += 1;
    }
}