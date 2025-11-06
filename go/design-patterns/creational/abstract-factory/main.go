package main

import "fmt"

func main() {
	adidasFactory, err := GetSportsFactory("adidas")
	if err != nil {
		panic(err)
	}

	nikeFactory, err := GetSportsFactory("nike")
	if err != nil {
		panic(err)
	}

	nikeShoe := nikeFactory.makeShoe()
	nikeShirt := nikeFactory.makeShirt()

	adidasShoe := adidasFactory.makeShoe()
	adidasShirt := adidasFactory.makeShoe()

	printShoeDetails(nikeShoe)
	printShoeDetails(adidasShoe)
	printShirtDetails(nikeShirt)
	printShirtDetails(adidasShirt)
}

func printShoeDetails(s IShoe) {
	fmt.Printf("=== SHOE ===\n")
	fmt.Printf("Logo: %s\n", s.getLogo())
	fmt.Printf("Size: %d\n", s.getSize())
	fmt.Printf("============\n")
}

func printShirtDetails(s IShirt) {
	fmt.Printf("=== SHIRT ===\n")
	fmt.Printf("Logo: %s\n", s.getLogo())
	fmt.Printf("Size: %d\n", s.getSize())
	fmt.Printf("=============\n")
}
