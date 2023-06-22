# oak-frontend-python

Oak 项目的 Python 语言前端。

## 功能介绍

负责将 Python 脚本解析并通过意图构造器直接归约为 Oak 的意图流。重点在于处理 Python 的动态特性并尝试提取其潜在的计算意图。

## 模块结构

- `lib.rs`: Python 语义到意图流的映射逻辑。

## 维护指南

1. **库映射**: 为常用的 Python 库（如 NumPy）提供意图层面的映射支持。
