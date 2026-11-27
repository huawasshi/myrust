
fn main () {
    // FnOnce 闭包：消耗捕获的变量，只能调用一次
    let data = String::from("RUNOOB");
    let consume = move || {
        let _ = data; // 将 data 移出闭包环境，消耗了它
        println!("data : {}", data);
        println!("data has been consumed");
    };
    consume();
    consume(); // 若取消注释，编译报错：FnOnce 闭包只能调用一次

    let add_ten = make_adder(-10);
    println!("10 + 2 = {}", add_ten(2)); // 输出: 10 + 2 = 12

    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s

    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s1);

    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    if x > 0 {
        Box::new(move |y| x + y)
    } else {
        Box::new(move |y| y - x)
    }
}

fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放