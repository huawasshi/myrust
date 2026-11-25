#![allow(dead_code, unused_variables, non_upper_case_globals)]

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ========== 用户自定义：结构体 ==========
struct Point { x: i32, y: i32 }   // 具名字段结构体
struct Pair(i32, i32);            // 元组结构体
struct Unit;                      // 单元结构体

// ========== 用户自定义：枚举 ==========
enum Color { Red, Green, Blue }
enum Shape {
    Circle(f64),
    Rectangle { w: f64, h: f64 },
}

// ========== 泛型结构体 ==========
struct Wrapper<T> { value: T }

// ========== 类型别名 ==========
type UserId = u64;

// ========== Never 类型函数 ==========
fn diverge() -> ! { panic!("永不返回") }

// ========== 函数与闭包示例 ==========
fn add(a: i32, b: i32) -> i32 { a + b }
fn identity<T>(x: T) -> T { x }

fn main() {
    // ---------- 1. 整数类型 ----------
    let a_i8: i8 = -128;
    let a_i16: i16 = 32767;
    let a_i32: i32 = 42;              // 默认整数类型
    let a_i64: i64 = 9_000_000_000;
    let a_i128: i128 = 170141183460469231731687303715884105727;
    let a_isize: isize = -1;          // 指针宽度

    let a_u8: u8 = 255;
    let a_u16: u16 = 65535;
    let a_u32: u32 = 4_294_967_295;
    let a_u64: u64 = 18_446_744_073_709_551_615;
    let a_u128: u128 = 340282366920938463463374607431768211455;
    let a_usize: usize = 100;         // 常用于索引和长度

    // 字面量后缀
    let lit1 = 42i32;
    let lit2 = 255u8;
    let lit3 = 1_000usize;

    // ---------- 2. 浮点类型 ----------
    let f32_val: f32 = 3.14;
    let f64_val: f64 = 2.718281828;   // 默认浮点类型
    let f_suffix = 1.0_f64;

    // ---------- 3. 布尔类型 ----------
    let t: bool = true;
    let f: bool = false;

    // ---------- 4. 字符类型 ----------
    let c1: char = 'A';
    let c2: char = '😀';
    let c3: char = '中';

    // ---------- 5. 元组 ----------
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    let (x, y, z) = tup;              // 解构
    let first = tup.0;                // 索引访问
    let unit: () = ();                // 单元类型

    // ---------- 6. 数组 ----------
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros: [u8; 3] = [0; 3];      // [0, 0, 0]
    let arr_first = arr[0];

    // ---------- 7. 切片 ----------
    let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
    let str_slice: &str = "hello";    // 字符串切片

    // ---------- 8. 字符串类型 ----------
    let s1: &str = "字面量";                     // 借用字符串切片
    let s2: String = String::from("拥有所有权");   // 堆上可变字符串
    let s3: String = "abc".to_string();
    let s4: &str = &s2[..];                      // 从 String 借用

    // ---------- 9. 集合类型 ----------
    let v: Vec<i32> = vec![1, 2, 3];
    let dq: VecDeque<i32> = VecDeque::new();
    let ll: LinkedList<i32> = LinkedList::new();
    let hm: HashMap<String, i32> = HashMap::new();
    let hs: HashSet<i32> = HashSet::new();
    let bm: BTreeMap<i32, String> = BTreeMap::new();
    let bs: BTreeSet<i32> = BTreeSet::new();

    // ---------- 10. 智能指针与包装类型 ----------
    let boxed: Box<i32> = Box::new(5);
    let rc: Rc<i32> = Rc::new(5);
    let arc: Arc<i32> = Arc::new(5);
    let cell: RefCell<i32> = RefCell::new(5);
    let mutex: Mutex<i32> = Mutex::new(5);
    let cow: Cow<str> = Cow::Borrowed("hi");

    // ---------- 11. 结构体实例 ----------
    let p = Point { x: 1, y: 2 };
    let q = Pair(3, 4);
    let u = Unit;

    // ---------- 12. 枚举实例 ----------
    let color = Color::Red;
    let shape1 = Shape::Circle(1.0);
    let shape2 = Shape::Rectangle { w: 2.0, h: 3.0 };

    // ---------- 13. 函数指针与闭包 ----------
    let fp: fn(i32, i32) -> i32 = add;
    let closure = |n: i32| n + 1;
    let boxed_closure: Box<dyn Fn(i32) -> i32> = Box::new(|n| n * 2);

    // ---------- 14. 引用与指针 ----------
    let num = 5;
    let r: &i32 = &num;               // 不可变引用
    let raw: *const i32 = &num;       // 裸指针
    let mut mnum = 10;
    let mr: &mut i32 = &mut mnum;     // 可变引用
    let raw_mut: *mut i32 = &mut mnum;

    // ---------- 15. Option 与 Result ----------
    let some: Option<i32> = Some(5);
    let none: Option<i32> = None;
    let ok: Result<i32, String> = Ok(1);
    let err: Result<i32, String> = Err("错误".to_string());

    // ---------- 16. 类型别名 ----------
    let id: UserId = 100;

    // ---------- 17. 其他标准库类型 ----------
    let dur: Duration = Duration::from_secs(5);
    let path: &Path = Path::new("/tmp");
    let pathbuf: PathBuf = PathBuf::from("/tmp/a.txt");
    let cstr: &CStr = c"hello";
    let cstring: CString = CString::new("hi").unwrap();

    // ---------- 18. 泛型 ----------
    let w: Wrapper<i32> = Wrapper { value: 1 };
    let n = identity(42);

    // 避免未使用变量警告
    println!("所有类型示例编译通过：{:?}", add(a_i32, lit1));
}