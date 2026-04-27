// clone = 深拷贝，两个都能用
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 复印一份

    println!("s1 = {}, s2 = {}", s1, s2);  // 都能用
}
