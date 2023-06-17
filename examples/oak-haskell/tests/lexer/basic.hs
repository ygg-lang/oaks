-- Basic Haskell syntax test file

-- Function definitions with type signatures
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

-- List operations
quickSort :: (Ord a) => [a] -> [a]
quickSort [] = []
quickSort (x:xs) = 
    quickSort [y | y <- xs, y <= x] ++ [x] ++ quickSort [y | y <- xs, y > x]

-- Data type definitions
data Tree a = Empty | Node a (Tree a) (Tree a) deriving (Show, Eq)

-- Type classes
class Describable a where
    describe :: a -> String

instance Describable Bool where
    describe True = "A true value"
    describe False = "A false value"

-- Higher-order functions
map' :: (a -> b) -> [a] -> [b]
map' _ [] = []
map' f (x:xs) = f x : map' f xs

-- Pattern matching
guardExample :: Int -> String
guardExample x
    | x < 0     = "Negative"
    | x == 0    = "Zero"
    | otherwise = "Positive"

-- List comprehensions
pythagoreanTriples :: [(Int, Int, Int)]
pythagoreanTriples = [(a, b, c) | c <- [1..100], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2]

-- Monadic operations
maybeExample :: Maybe Int -> Maybe Int
maybeExample mx = do
    x <- mx
    return (x * 2)

-- Record syntax
data Person = Person {
    name :: String,
    age :: Int,
    email :: String
} deriving (Show)

-- Lambda expressions
filterEven :: [Int] -> [Int]
filterEven = filter (\x -> x `mod` 2 == 0)

-- String operations
reverseWords :: String -> String
reverseWords = unwords . reverse . words

-- Main function
main :: IO ()
main = do
    putStrLn "Hello, Haskell!"
    print $ factorial 5
    print $ quickSort [3, 1, 4, 1, 5, 9, 2, 6]
    let person = Person "Alice" 30 "alice@example.com"
    print person