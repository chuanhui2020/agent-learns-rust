// loop 无限循环 + break 返回值
fn main() {
    // 基本 loop，用 break 跳出
    let mut i = 0;
    loop {
        i += 1;
        if i == 5 {
            break;
        }
    }
    println!("stopped at {}", i);

    // break 可以带返回值（Java 做不到）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);
}
