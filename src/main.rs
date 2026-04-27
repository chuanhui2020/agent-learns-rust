fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {}", s2);
    println!("s1 = {}", s1);  // s1 已经被 move 了
}
