// 不可变变量不能改，编译会报错
fn main() {
    let x = 5;
    x = 10;  // 报错：cannot assign twice to immutable variable
    println!("x = {}", x);
}
