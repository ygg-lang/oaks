module Basic exposing (..)

import Html exposing (text)
import List exposing (map)

-- Single line comment

{- Multi-line
   comment
-}

-- Type Alias
type alias User =
    { name : String
    , age : Int
    , isActive : Bool
    }

-- Custom Type
type Status
    = Active
    | Inactive
    | Pending Int

-- Function with type annotation
add : Int -> Int -> Int
add a b =
    a + b

-- Pattern matching
describeStatus : Status -> String
describeStatus status =
    case status of
        Active ->
            "User is active"

        Inactive ->
            "User is inactive"

        Pending days ->
            "Pending for " ++ String.fromInt days ++ " days"

-- Let expression
calculate : Int -> Int
calculate x =
    let
        y = x * 2
        z = y + 10
    in
    z * 2

-- Record update
updateAge : User -> User
updateAge user =
    { user | age = user.age + 1 }

-- List operations
processList : List Int -> List Int
processList numbers =
    numbers
        |> map (\n -> n * 2)
        |> map (\n -> n + 1)

-- Main function
main =
    text "Hello, Elm!"
