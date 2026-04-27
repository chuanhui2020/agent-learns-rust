// while 循环
fn main() {
    let mut count = 3;
    doWhileLoop(count);
}

fn doWhileLoop(count: i32) {
    while count > 0 {
        println!("{}.. ", count);
        count -= 1;
    }
    println!("go!");
}
