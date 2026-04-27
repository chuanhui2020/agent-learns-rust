// 借用规则：
// 1. 不可变借用（&）可以同时有多个
// 2. 可变借用（&mut）同一时间只能有一个
// 3. 不可变借用和可变借用不能同时存在
fn main() {
    let mut s = String::from("hello");

    // 多个不可变借用 — OK
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // 可变借用 — r1 r2 用完了，所以这里 OK
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
