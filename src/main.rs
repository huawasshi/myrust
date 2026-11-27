
use std::ffi::c_int;
use std::thread;

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
        email: String::from("someone@examples.com"),
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

    // 闭包：省略类型注解，编译器自动推断
    let add_one = |x| x + 1;
    println!("add_one(4) = {}", add_one(4)); // 输出: 5

    // 闭包：显式标注参数和返回值类型
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("multiply(3, 7) = {}", multiply(3, 7)); // 输出: 21

    // 多行闭包需要加大括号
    let greet = |name: &str| {
        let greeting = format!("Hello, {}!", name);
        greeting
    };
    println!("{}", greet("runoob")); // 输出: Hello, runoob!

    let text = String::from("runoob");
    // 闭包按引用捕获 text，不获取所有权
    let print_text = | x | println!("text = {}", * &text);
    print_text(1); // 输出: text = runoob
    // 借用已结束，外部仍然可以使用 text
    println!("外部仍可使用: {}", text);

    let mut counter = 0;
    // 闭包以 &mut 方式捕获 counter，闭包本身也要声明为 mut
    let mut inc = || {
        counter += 1;
    };
    inc();
    inc();
    println!("counter = {}", counter); // 输出: counter = 2

    let mut a = "1".to_string();
    a = a + &"2".to_string();
    a = a + "3";
    a.push_str("4");
    let b = "11".to_string();
    {let a1 = &mut a;
    a1.push_str("5");
    println!("a1 = {}", a1);
        let slice = a1.get(1..3);      // 切片，得到 "23" (类型是 &str)
        println!("slice = {:?}", slice.unwrap_or_else(|| ""));
        let slice = &a1[1..3];      // 切片，得到 "23" (类型是 &str)
        println!("slice = {:?}", slice);
        for c in a1.chars() {       // 按字符迭代
            println!("{}", c);
        }
    }
    println!("a = {}", a);

    let owned = String::from("RUNOOB");
    // move 将 owned 的所有权转移进闭包
    let take_owned = move || println!("owned = {}", owned);
    take_owned(); // 输出: owned = RUNOOB
    // 若取消下面这行注释，将编译报错：owned 的所有权已被移入闭包
    // println!("{}", owned);

    // FnOnce 闭包：消耗捕获的变量，只能调用一次
    let data = String::from("RUNOOB");
    let consume = move || {
        let _ = data; // 将 data 移出闭包环境，消耗了它
        println!("data has been consumed");
    };
    println!("data: {}", data);
    consume();
    println!("data: {}", data);
    consume(); // 若取消注释，编译报错：FnOnce 闭包只能调用一次
    println!("data: {}", data);

    let double = |x: i32| x.pow(2) * 2;
    let result = apply(5, double);
    println!("Result: {}", result); // 输出: Result: 10

    // 也可以直接传入匿名闭包
    let result2 = apply(3, |x| x + 100);
    println!("Result2: {}", result2); // 输出: Result2: 103

    let x = 5;
    let y = 3;
    let add_five = make_adder(x);
    println!("5 + 3 = {}", add_five(y)); // 输出: 5 + 3 = 8
    println!("x : {}", x);
    println!("y : {}", y);

    let add_ten = make_adder(10);
    println!("10 + 2 = {}", add_ten(2)); // 输出: 10 + 2 = 12

    let nums = vec![1, 2, 3, 4, 5];

    // 为每个数字创建一个线程，move 将 num 的所有权移入线程
    let handles: Vec<_> = nums.into_iter().map(|num| {
        thread::spawn(move || {
            num * 2
        })
    }).collect();

    // 等待所有线程完成并收集结果
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Result: {}", result);
    }

    let s = String::from("broadcast");

    let part1 = s[0..5].to_string();
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    let mut s = String::from("runoob");
    s.push_str("yes");
    s.push('!');
    let slice = &s[0..3];
    // s.push_str("yes!"); // 错误
    println!("slice = {}", slice);
}

fn apply<F>(val: i32, f: F) -> i32
where
    F: Fn(i32) -> i32, // F 必须实现 Fn(i32) -> i32
{
    f(val)
}

// 返回一个闭包，捕获参数 x 并与传入值相加
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    // 必须 move，否则 x 是局部引用，闭包返回后 x 已失效
    move |y| x + y
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