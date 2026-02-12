-- Comprehensive Haskell Lexer Test
module Main where

import Data.List (sort, group)
import qualified Data.Map as Map
import Control.Monad (when, forM_)

-- Basic Types
i :: Int
i = 42

d :: Double
d = 3.14

c :: Char
c = 'A'

s :: String
s = "Hello, Haskell!"

b :: Bool
b = True

-- Lists and Tuples
list :: [Int]
list = [1, 2, 3, 4, 5]

tuple :: (Int, String)
tuple = (1, "One")

-- Algebraic Data Types
data Color
    = Red
    | Green
    | Blue
    | RGB Int Int Int
    deriving (Show, Eq)

data Maybe a
    = Nothing
    | Just a

-- Record Syntax
data Person = Person
    { name :: String
    , age :: Int
    , active :: Bool
    } deriving (Show)

-- Type Classes
class Describable a where
    describe :: a -> String

instance Describable Color where
    describe Red = "Red color"
    describe Green = "Green color"
    describe Blue = "Blue color"
    describe (RGB r g b) = "RGB(" ++ show r ++ "," ++ show g ++ "," ++ show b ++ ")"

-- Functions
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

-- Pattern Matching
analyzeList :: [a] -> String
analyzeList [] = "Empty list"
analyzeList [x] = "Singleton list"
analyzeList (x:xs) = "List with head and tail"

-- Guards
grade :: Int -> String
grade score
    | score >= 90 = "A"
    | score >= 80 = "B"
    | score >= 70 = "C"
    | otherwise   = "F"

-- Case Expressions
checkZero :: Int -> String
checkZero x = case x of
    0 -> "It's zero"
    _ -> "Not zero"

-- Where Clause
circleArea :: Double -> Double
circleArea r = pi * r ^ 2
    where
        pi = 3.14159

-- Let Expression
cylinderVolume :: Double -> Double -> Double
cylinderVolume r h =
    let area = circleArea r
    in area * h

-- Higher Order Functions
squares :: [Int]
squares = map (\x -> x * x) [1..10]

filtered :: [Int]
filtered = filter (> 5) squares

-- List Comprehensions
pairs :: [(Int, Int)]
pairs = [(x, y) | x <- [1..3], y <- [1..3], x /= y]

-- Operator Sections
increment :: Int -> Int
increment = (+ 1)

half :: Double -> Double
half = (/ 2)

-- Infix Operators
customOp :: Int -> Int -> Int
customOp x y = x + y * 2

result = 5 `customOp` 3

-- Monads and Do Notation
main :: IO ()
main = do
    putStrLn "What is your name?"
    name <- getLine
    putStrLn $ "Hello, " ++ name
    
    when (length name > 5) $ do
        putStrLn "That's a long name!"

-- Newtype
newtype UserId = UserId Int

-- Type Synonyms
type Username = String

-- Comments
-- Single line comment
{- Multi-line
   comment -}
{- Nested {- comments -} -}

-- Pragma
{-# LANGUAGE OverloadedStrings #-}
{-# OPTIONS_GHC -Wall #-}

-- Foreign Function Interface
foreign import ccall "math.h sin" c_sin :: Double -> Double

-- GADTs
data Expr a where
    I   :: Int  -> Expr Int
    B   :: Bool -> Expr Bool
    Add :: Expr Int -> Expr Int -> Expr Int
    Mul :: Expr Int -> Expr Int -> Expr Int
    Eq  :: Eq a => Expr a -> Expr a -> Expr Bool
