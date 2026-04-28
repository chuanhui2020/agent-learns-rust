// 借用与引用（四）：实战中的常见借用模式

fn main() {
    // 模式 1：函数同时借用多个参数
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let longer = longest(&s1, &s2);
    println!("更长的是: {}", longer);

    // 模式 2：借用集合中的元素
    let names = vec!["Alice", "Bob", "Charlie"];
    let first = &names[0]; // 借用 Vec 中的元素
    println!("第一个: {}", first);
    // 注意：这里不能对 names 做 push 等修改操作
    // 因为 first 还在借用 names 的数据
    // names.push("Dave"); // 取消注释会编译失败！

    // 模式 3：方法中的 &self 和 &mut self
    let mut counter = Counter { value: 0 };
    println!("当前值: {}", counter.get());  // &self 借用
    counter.increment();                     // &mut self 借用
    counter.increment();
    println!("增加后: {}", counter.get());

    // 模式 4：引用的引用（自动解引用）
    let s = String::from("rust");
    let r1 = &s;
    let r2 = &r1; // &&String
    // Rust 会自动解引用，所以可以直接调用 String 的方法
    println!("长度: {}", r2.len());
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 这里的 'a 是生命周期标注，后面会详细学
    // 现在只需知道：返回的引用和输入的引用"活一样久"
    if s1.len() >= s2.len() { s1 } else { s2 }
}

struct Counter {
    value: i32,
}

impl Counter {
    fn get(&self) -> i32 {        // &self = 不可变借用自己
        self.value
    }
    fn increment(&mut self) {     // &mut self = 可变借用自己
        self.value += 1;
    }
}
