// String 赋值 = 移动（move），原来的不能用了
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 的所有权转给了 s2

    println!("s2 = {}", s2);  // OK
    // println!("s1 = {}", s1);  // 取消注释会报错：s1 已经被 move 了
}
