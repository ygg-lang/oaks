module Main where

import Prelude

-- 基本函数定义
add :: Int -> Int -> Int
add x y = x + y

-- 字符串字面量
greeting :: String
greeting = "Hello, PureScript!"

-- 数字字面量
numbers :: Array Number
numbers = [1.0, 2.5, 3.14]

-- 布尔值
isTrue :: Boolean
isTrue = true

-- 字符字面量
char :: Char
char = 'a'

-- 操作符
result :: Int
result = 10 + 20 * 30 / 5 - 2

-- 注释测试
{- 多行注释
   可以跨行 -}
main :: Effect Unit
main = log greeting