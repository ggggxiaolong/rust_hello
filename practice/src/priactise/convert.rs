use std::io;

enum TempType {
    C(i32),
    F(i32),
}

impl TempType{
    fn convert(&self) -> i32 {
        return match self {
            TempType::C(temp) => (*temp as f64 * 1.8).round() as i32 + 32,
            TempType::F(temp) => ((*temp - 32) as f64 / 1.8).round() as i32,
        };
    }
}

pub fn print_convert(){
    println!("转换摄氏与华氏");
    println!("输入温度：");
    let mut temp = String::new();
    let mut t = String::new();
    io::stdin().read_line(&mut temp).expect("Fail to read line");
    println!("输入类别 摄氏度（1） 华氏度（2）：");
    io::stdin().read_line(&mut t).expect("Fail to read line");
    let temp: i32 = temp.trim().parse().expect("Fail input temp");
    let t: i32 = t.trim().parse().expect("Failr input type");
    let t = match t {
        1 => TempType::C(temp),
        _ => TempType::F(temp),
    };
    match t {
        TempType::C(temp) => println!("{}℃ = {}℉", temp, t.convert()),
        TempType::F(temp) => println!("{}℉ = {}℃", temp, t.convert()),
    }
}
