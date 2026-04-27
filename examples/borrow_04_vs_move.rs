// 对比：move vs 借用
fn main() {
    // move — 所有权转走，不能再用
    let s1 = String::from("move me");
    take_it(s1);
    // println!("{}", s1);  // 编译错误

    // 借用 — 所有权不变，还能用
    let s2 = String::from("borrow me");
    look_at(&s2);
    println!("still mine: {}", s2);  // OK
}

fn take_it(s: String) {
    println!("took: {}", s);
}

fn look_at(s: &String) {
    println!("looked: {}", s);
}
