// 基本类型
fn main() {
    // 整数
    let a: i32 = 42;    // 有符号 32 位，类似 Java 的 int
    let b: i64 = 100;   // 64 位，类似 Java 的 long
    let c: u32 = 42;    // 无符号 32 位（Java 没有无符号类型）

    // 浮点
    let d: f64 = 3.14;  // 类似 Java 的 double

    // 布尔
    let e: bool = true;  // 和 Java 一样

    // 字符
    let f: char = 'A';   // 和 Java 一样用单引号

    println!("i32: {}", a);
    println!("i64: {}", b);
    println!("u32: {}", c);
    println!("f64: {}", d);
    println!("bool: {}", e);
    println!("char: {}", f);

    // 大多数时候不用写类型，编译器会自动推断
    let g = 42;    // 自动推断为 i32
    let h = 3.14;  // 自动推断为 f64
    println!("inferred: {} {}", g, h);
}
