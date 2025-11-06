package main

import "fmt"

func main() {
	ak47, err := getGun("ak47")
	if err != nil {
		panic(err)
	}

	musket, err := getGun("musket")
	if err != nil {
		panic(err)
	}

	// _, err = getGun("glock")
	// if err != nil {
	// 	panic(err)
	// }

	printDetails(ak47)
	printDetails(musket)
}

func printDetails(g IGun) {
	fmt.Printf("Gun: %s\n", g.getName())
	fmt.Printf("Power: %d\n", g.getPower())
}
