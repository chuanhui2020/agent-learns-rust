// 借用与引用（二）：可变引用 &mut T
//
// 不可变引用 &T 只能读，不能改。
// 如果想通过引用修改数据，需要用 &mut T。

fn main() {
    // 变量本身必须是 mut，才能创建可变引用
    let mut s = String::from("hello");

    println!("修改前: {}", s);
    append_world(&mut s);
    println!("修改后: {}", s);

    // 核心规则：同一时间只能有一个可变引用
    let r1 = &mut s;
    r1.push_str("!!");
    println!("r1 修改后: {}", r1);
    // 如果这里再创建 let r2 = &mut s; 并同时使用 r1，编译器会报错
    // Rust 用这个规则在编译期防止数据竞争（data race）

    // 另一个规则：不可变引用和可变引用不能同时存在
    let mut n = 42;
    let r_immut = &n;
    println!("不可变引用: {}", r_immut);
    // r_immut 最后一次使用在这里，之后编译器认为它已经"结束"了
    // 所以下面可以创建可变引用（这叫 NLL —— Non-Lexical Lifetimes）
    let r_mut = &mut n;
    *r_mut += 1;
    println!("可变引用修改后: {}", n);
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}
