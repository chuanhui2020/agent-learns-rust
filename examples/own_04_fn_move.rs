// 函数传参也会 move
fn main() {
    let msg = String::from("hi");
    take_it(msg);  // msg 的所有权转进了函数
    // println!("{}", msg);  // 取消注释会报错：msg 已经被 move 了
}

fn take_it(s: String) {
    println!("I took: {}", s);
    // 函数结束，s 被自动释放
}
