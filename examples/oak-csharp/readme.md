# oak-frontend-csharp

Oak 项目的 C# 语言前端。

## 功能介绍

负责将 C# 源代码解析并通过意图构造器直接归约为 Oak 的意图流。

## 模块结构

- `lib.rs`: C# 语法解析逻辑。

## 维护指南

1. **LINQ 支持**: 将 LINQ 查询直接映射到高性能的 UIR 意图序列。
