# Agent Learns Rust

三天 Rust 速成计划，通过 Claude 协作学习。

## 用户背景

- 有 Java / Go / JS 项目经验，非编程新手
- 用这些语言做类比来加速理解 Rust 概念

## 教学方式

- 一个知识点一个知识点来，不要信息轰炸
- 讲完一个概念后停下来，等用户说"下一个"才继续
- 不要自动推进，不要替用户做练习题，让用户掌控节奏
- 每个知识点写成独立的 example 文件，用 `cargo run --example <name>` 运行

## 学习进度

### Day 1 — 基础与所有权
- [x] 环境搭建 & Hello World
- [x] 变量与类型（let, mut, 基本类型）
- [x] 函数（fn, 返回值, &str 参数）
- [x] 控制流（if, for, while, loop）
- [x] 所有权（copy, move, clone）
- [ ] 借用与引用（例子已生成 borrow_01 ~ borrow_04，待学习）
- [ ] 切片（&[T], &str）
- [ ] 字符串深入（String vs &str, UTF-8, 常用转换方法）
- [ ] 生命周期基础（'a 标注、函数签名中的生命周期、struct 中的生命周期、'static）
- [ ] 练习

### Day 2 — 类型系统与核心特性
- [ ] Struct & 方法（关联函数、impl 块）
- [ ] Enum & 模式匹配基础（match, if let, while let）
- [ ] 模式匹配进阶（解构、guards、@ 绑定、matches! 宏）
- [ ] Option / Result / 错误处理基础（unwrap, expect, ?）
- [ ] 错误处理进阶（自定义错误类型、thiserror/anyhow）
- [ ] Trait & 泛型（trait bound, where 子句）
- [ ] 常用 Trait（Display, Debug, Clone, Copy, Drop, Deref, Default）
- [ ] Trait 对象与动态分发（dyn Trait, Box<dyn Trait>）
- [ ] 类型转换（From/Into, AsRef, TryFrom/TryInto）
- [ ] 集合（Vec, HashMap, String）
- [ ] 闭包与迭代器（Fn/FnMut/FnOnce, Iterator trait, 常用适配器）
- [ ] 智能指针（Box, Rc, RefCell, Cow）
- [ ] 练习

### Day 3 — 工程能力与并发
- [ ] 模块系统 & Crate 与依赖（mod, pub, use, Cargo.toml）
- [ ] 测试（#[test], cargo test, 集成测试）
- [ ] 文件 IO（std::fs, std::io, BufReader/BufWriter）
- [ ] 多线程（std::thread, channel, move 闭包）
- [ ] 共享状态并发（Arc, Mutex, RwLock）
- [ ] async/await 基础（tokio, Future trait）
- [ ] 宏基础（macro_rules!, 声明宏 vs 过程宏概念）
- [ ] unsafe 基础（裸指针、unsafe fn、FFI 概念）
- [ ] 实战项目

## 项目结构

- `src/main.rs` — 主程序（当前为 hello world）
- `examples/` — 每个知识点的独立示例

## 示例命名规则

- `var_*` — 变量与类型
- `fn_*` — 函数
- `flow_*` — 控制流
- `own_*` — 所有权
- `borrow_*` — 借用与引用
- `slice_*` — 切片
- `str_*` — 字符串深入
- `life_*` — 生命周期
- `struct_*` — Struct & 方法
- `enum_*` — Enum & 模式匹配
- `pattern_*` — 模式匹配进阶
- `err_*` — Option / Result / 错误处理
- `trait_*` — Trait & 泛型
- `common_trait_*` — 常用 Trait
- `dyn_*` — Trait 对象与动态分发
- `convert_*` — 类型转换
- `col_*` — 集合
- `closure_*` — 闭包与迭代器
- `ptr_*` — 智能指针
- `mod_*` — 模块系统
- `test_*` — 测试
- `io_*` — 文件 IO
- `thread_*` — 多线程与并发
- `async_*` — async/await
- `macro_*` — 宏
- `unsafe_*` — unsafe
