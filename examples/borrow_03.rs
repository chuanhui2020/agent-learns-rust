// 借用与引用（三）：悬垂引用 —— Rust 不让你犯的错
//
// 在 C/C++ 里，函数返回一个指向局部变量的指针是经典 bug。
// Go 没这个问题，因为编译器会自动把变量逃逸到堆上。
// Rust 的做法：直接编译报错，不让你写出悬垂引用。

fn main() {
    // 正确做法：直接返回 String（转移所有权出去）
    let s = no_dangle();
    println!("正确返回: {}", s);

    // 下面这个函数如果取消注释，编译器会报错：
    // "returns a reference to data owned by the current function"
    // let s2 = dangle();
}

// 错误示例（取消注释会编译失败）：
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // s 在函数结束时被释放，返回的引用指向了已释放的内存！
// }

// 正确做法：返回值本身，把所有权转移给调用者
fn no_dangle() -> String {
    let s = String::from("hello");
    s // 所有权 move 给调用者，没有悬垂问题
}
