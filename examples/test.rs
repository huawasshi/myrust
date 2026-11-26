
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
}

fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    if x > 0 {
        Box::new(move |y| x + y)
    } else {
        Box::new(move |y| y - x)
    }
}