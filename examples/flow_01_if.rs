// if 基础 + if 表达式
fn main() {
    let n = 7;

    // if 不需要括号
    if n > 5 {
        println!("{} is big", n);
    } else if n > 0 {
        println!("{} is small", n);
    } else {
        println!("{} is negative", n);
    }

    // if 是表达式，可以赋值（类似 Java 的三元运算符）
    let label = if n > 5 { "big" } else { "small" };
    let result = if n > 10 { "too big" } else { "ok" };
    println!("{} is {}", n, label);
    println!("result = {}", result);
}
