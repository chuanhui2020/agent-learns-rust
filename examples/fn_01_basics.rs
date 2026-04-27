// 函数基础：定义和调用
fn main() {
    greet("Rust");

    let result = add(3, 7);
    println!("3 + 7 = {}", result);
}

// 没有返回值，类似 Java 的 void
fn greet(name: &str) {
    println!("hello, {}!", name);
}

// -> 后面是返回类型，最后一行不加分号 = 返回值
fn add(a: i32, b: i32) -> i32 {
    a + b
}
