// 借用：用 & 借来看，不拿走所有权
fn main() {
    let s = String::from("hello");

    // &s = 借给函数看，所有权还是我的
    print_it(&s);
    println!("I still own: {}", s);  // 还能用
}

// &String = 借来的 String，只能读
fn print_it(s: &String) {
    println!("borrowed: {}", s);
}
