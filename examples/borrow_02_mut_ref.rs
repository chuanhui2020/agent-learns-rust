// 可变借用：用 &mut 借来可以改
fn main() {
    let mut s = String::from("hello");

    add_world(&mut s);  // 借给函数，允许修改
    println!("{}", s);   // "hello world"
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
