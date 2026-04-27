// 所有权规则：每个值有且只有一个 owner
// 基本类型赋值 = 复制，两个都能用
fn main() {
    let a = 5;
    let b = a;  // 复制了一份
    println!("a = {}, b = {}", a, b);  // 都能用
}
