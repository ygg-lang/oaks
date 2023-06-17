package com.example

import kotlin.math.*

// 基本类定义
class Person(val name: String, var age: Int) {
    // 方法定义
    fun greet(): String {
        return "Hello, my name is $name and I'm $age years old"
    }
    
    // 属性
    val isAdult: Boolean
        get() = age >= 18
}

// 函数定义
fun add(x: Int, y: Int): Int = x + y

// 变量声明
val pi = 3.14159
var counter = 0

// 字符串字面量
val greeting = "Hello, Kotlin!"
val multiline = """
    This is a
    multiline string
""".trimIndent()

// 字符字面量
val char = 'K'

// 数字字面量
val integer = 42
val long = 42L
val float = 3.14f
val double = 3.14159

// 布尔值
val isTrue = true
val isFalse = false

// 空值
val nullable: String? = null

// 操作符
val result = 10 + 20 * 30 / 5 - 2

// 注释测试
/* 多行注释
   可以跨行 */
fun main() {
    println(greeting)
}