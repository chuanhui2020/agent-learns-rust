// for 循环
fn main() {
    // 遍历范围：0 到 4（不含 5）
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // 含右端点：0 到 5（含 5）
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    for j in 1..=10 {
        println!("{}", j);
    }

    // 遍历数组，类似 Java 的 for (String s : arr)
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        print!("{} ", fruit);
    }
    println!();
}
