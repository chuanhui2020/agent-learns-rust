// return 关键字：提前返回
fn main() {
    println!("max of 3, 7 = {}", max(3, 7));
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;  // 提前返回用 return
    }
    b  // 最后一行不加分号 = 返回值
}
