// 切片：引用数组/Vec 的一部分，不拥有数据
fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let slice = &nums[1..4]; // 取索引 1, 2, 3（左闭右开）
    println!("slice = {:?}", slice); // [2, 3, 4]

    println!("whole = {:?}", nums); // 原数据还在
}
