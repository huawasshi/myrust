pub mod AllTypes;

use std::ffi::c_int;

///cargo new <project-name>：创建一个新的 Rust 项目。
///cargo build：编译当前项目。
///cargo run：编译并运行当前项目。
///cargo check：检查当前项目的语法和类型错误。
///cargo test：运行当前项目的单元测试。
///cargo update：更新 Cargo.toml 中指定的依赖项到最新版本。
///cargo --help：查看 Cargo 的帮助信息。
///cargo publish：将 Rust 项目发布到 crates.io。
///cargo clean：清理构建过程中生成的临时文件和目录

fn main() {
    let a = 1;
    const B:i32 = 2i32;
    let c = add(a, B);
    println!("{}", c);
    if c < 1 {
        println!("c is <1");
    }else if c == 1 {
        println!("c is =1");
    }else {
        println!("c is >1");
    }
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权被转移给了 s2
    // println!("{}", s1); // 此处编译会报错，因为 s1 已不再拥有该值
    let s = String::from("hello");
    let s3 = &s;
    println!("{}", s);
    println!("{}", s3);
    let len = calculate_length(&s);  // 借用
    println!("The length of '{}' is {}.", s, len);

    let user1 = User {
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{}", user1.email);

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{}", value_in_cents(Coin::Dime));

    let v = vec![10, 20, 30, 40, 50];

    // 情况 1：索引合法，返回 Some
    let r1 = get_element(2, &v);
    println!("get_element(2, &v) = {:?}", r1);   // Some(30)

    // 情况 2：索引越界，返回 None
    let r2 = get_element(10, &v);
    println!("get_element(10, &v) = {:?}", r2);  // None

    // 用 match 显式处理两种情况
    match get_element(1, &v) {
        Some(x) => println!("索引 1 的值是 {}", x),   // 输出：索引 1 的值是 20
        None => println!("索引 1 越界了"),
    }

    match get_element(99, &v) {
        Some(x) => println!("索引 99 的值是 {}", x),
        None => println!("索引 99 越界了"),           // 输出：索引 99 越界了
    }

    // 用 unwrap_or 给越界情况一个默认值
    let value = get_element(100, &v).unwrap_or(-1);
    println!("索引 100 的值（越界默认 -1）= {}", value); // -1

    // 用 if let 只处理有值的情况
    if let Some(x) = get_element(0, &v) {
        println!("索引 0 的值是 {}", x);              // 10
    }

    let a = 10;
    let b = &a;
    println!("*b = {}", *b);
    println!("b = {:p}", b);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = &tup.1;
    let tup2 = &tup.2;
    println!("tup1 = {:p}", tup1);
    println!("tup2 = {:p}", tup2);

    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1
    };
    println!(" \'O\' 的索引为 {}", location);

    let a = String::from("123");
    let b = &a;
    let c = &a;
    println!("a = {}, b = {}, c = {}", a, b, c);

    let vec = vec![1, 2, 3, 4, 5];
    let squared_vec: Vec<i32> = vec.iter().map(|&x| x * x).collect();
    //let squared_vec: Vec<i32> = vec.iter().map(|x| *x * *x).collect();
    let fold_vec: Vec<i32> = vec.iter().fold(Vec::new(), |mut acc, x| {acc.push((*x).pow(3)); acc});
    let take_vec: Vec<&i32> = vec.iter().take(3).collect();
    let skip_vec: Vec<i32> = vec.iter().skip(1).skip(1).cloned().collect();
    let enumerate_vec: Vec<(usize, &i32)> = vec.iter().enumerate().collect();
    // 编译器认为引用生命周期到println!后仍然未结束，不允许被借用后移动所有权
    // let filtered_vec: Vec<i32> = vec.into_iter().filter(|&x| x % 2 == 0).collect();

    println!("squared_vec = {:?}", squared_vec);
    println!("fold_vec = {:?}", fold_vec);
    println!("take_vec = {:?}", take_vec);
    println!("skip_vec = {:?}", skip_vec);
    println!("enumerate_vec = {:?}", enumerate_vec);

    //println!("filtered_vec = {:?}", filtered_vec);
    let vec = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()];
    for num in vec.iter() {
        println!("{:p}", num);
        println!("{:?}", num);
    }
    for num in vec.into_iter() {
        println!("{:?}", num);
    }

    let arr = [1, 1, 2, 3, 4, 5];
    let mut iter = arr.into_iter().peekable();
    while let Some(val) = iter.next() {
        if val % 2 == 0 {
            continue;
        }
        println!("now: {}", val);
        println!("next: {:?}", iter.peek().copied().unwrap_or(-1));
    }

    let mut counter = Counter::new();
    while let Some(num) = counter.next() {
        println!("{}", num);  // 输出 1 到 5
    }
}
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub trait Iterator {

    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 其他默认实现的方法如 map, filter 等。
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn get_element(index: usize, vec: &Vec<i32>) -> Option<i32> {
    if index < vec.len() {
        Some(vec[index])
    } else {
        None
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add(a:c_int, b:i32) -> i32 {
    let c = a + b;
    return c;
}

fn counter() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter: {}", counter);
        if counter == 10 {
            break;
        }
    }
    let mut number = 3;
    while number != 0 || number < 5 {
        println!("{}!", number);
        number -= 1;
    }
    for number in 1..4 {
        println!("{}!", number);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}