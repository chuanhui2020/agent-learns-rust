# Agent Learns Rust

三天 Rust 速成计划，通过 Claude 协作学习。

## 学习进度

### Day 1 — 基础与所有权
- [x] 环境搭建 & Hello World
- [x] 变量与类型（let, mut, 基本类型）
- [x] 函数（fn, 返回值, &str 参数）
- [x] 控制流（if, for, while, loop）
- [ ] 所有权（正在学习：own_01 ~ own_04）
- [ ] 借用与引用

### Day 2 — 类型系统与错误处理
- [ ] Struct & 方法
- [ ] Enum & 模式匹配
- [ ] Option / Result / 错误处理
- [ ] Trait
- [ ] 泛型
- [ ] 集合

### Day 3 — 工程能力与实战
- [ ] 模块系统
- [ ] Crate 与依赖
- [ ] 闭包与迭代器
- [ ] 智能指针
- [ ] 实战项目

## 项目结构

- `src/main.rs` — 主程序（当前为 hello world）
- `examples/` — 每个知识点的独立示例，用 `cargo run --example <name>` 运行

## 示例命名规则

- `var_*` — 变量与类型
- `fn_*` — 函数
- `flow_*` — 控制流
- `own_*` — 所有权
