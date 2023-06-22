# oak-frontend-typescript

Oak 项目的 TypeScript 语言前端。

## 功能介绍

负责将 TypeScript/JavaScript 代码解析并通过意图构造器直接归约为 Oak 的意图流。

## 模块结构

- `lib.rs`: 基于 tree-sitter 或自定义解析器的 TS 解析逻辑。

## 维护指南

1. **类型提取**: 尝试从 TS 类型定义中提取有助于 UIR 优化的元数据。
