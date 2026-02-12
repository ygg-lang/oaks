-- Comprehensive PureScript Lexer Test

module Main where

import Prelude
import Effect (Effect)
import Effect.Console (log)
import Data.Maybe (Maybe(..))
import Data.List (List(..), (:))
import Data.Tuple (Tuple(..))
import Data.Either (Either(..))
import Control.Monad.Eff (Eff)

-- Basic Types
i :: Int
i = 42

n :: Number
n = 3.14

s :: String
s = "Hello, world!"

b :: Boolean
b = true

c :: Char
c = 'A'

-- Arrays and Records
arr :: Array Int
arr = [1, 2, 3, 4]

rec :: { name :: String, age :: Int }
rec = { name: "Alice", age: 30 }

-- Row Polymorphism
showPerson :: forall r. { name :: String | r } -> String
showPerson p = "Name: " <> p.name

-- Algebraic Data Types
data Shape
    = Circle Number
    | Rectangle Number Number
    | Point

-- Pattern Matching
area :: Shape -> Number
area (Circle r) = 3.14159 * r * r
area (Rectangle w h) = w * h
area Point = 0.0

-- Newtypes
newtype Email = Email String

-- Type Classes
class Show a where
    show :: a -> String

instance showShape :: Show Shape where
    show (Circle r) = "Circle " <> show r
    show (Rectangle w h) = "Rectangle " <> show w <> " " <> show h
    show Point = "Point"

-- Functional Dependencies
class Collection c e | c -> e where
    insert :: e -> c -> c
    member :: e -> c -> Boolean

-- Kind Signatures
data Proxy (a :: Type) = Proxy

-- Foreign Imports
foreign import logMessage :: String -> Effect Unit

-- Do Notation
main :: Effect Unit
main = do
    log "Starting..."
    let result = area (Circle 10.0)
    log $ "Area: " <> show result
    pure unit

-- Operators
infixl 4 add as +
infixr 5 append as <>
