package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}

func add(x, y int) int {
    return x + y
}

type Person struct {
    Name string
    Age  int
}

var numbers = []int{1, 2, 3, 4, 5}