package main

import (
	"fmt"
	"io"
)

func getPlay(nth string) (string, error) {
	var p string
	var err error

	for {
		fmt.Printf("Enter %s play\n", nth)
		_, err = fmt.Scan(&p)
		if err == io.EOF {
			break
		}
		if err != nil {
			err = fmt.Errorf("play read error: %w", err)
			break
		}
		if p != "r" && p != "p" && p != "s" {
			fmt.Printf("Invalid play char: %s\n", p)
			continue
		}
		break
	}

	return p, err
}

func expandPlay(p string) string {
	switch p {
	case "r":
		return "Rock"
	case "p":
		return "Paper"
	case "s":
		return "Scissors"
	default:
		return "Invalid"
	}
}

func resolvePlays(first string, second string) string {
	if first == second {
		return "Draw"
	}

	if (first == "r" && second == "s") || (first == "p" && second == "r") || (first == "s" && second == "p") {
		return "Win"
	}

	if (first == "r" && second == "p") || (first == "p" && second == "s") || (first == "s" && second == "r") {
		return "Loss"
	}

	return "Unknown"
}

func main() {
	i := 1
	for {
		fmt.Printf("Turn %d:\n", i)

		first, err := getPlay("first")
		if err != nil {
			break
		}
		fmt.Println(expandPlay(first))

		second, err := getPlay("second")
		if err != nil {
			break
		}
		fmt.Println(expandPlay(second))

		outcome := resolvePlays(first, second)
		fmt.Printf("Outcome: %s\n", outcome)
		if outcome != "Draw" {
			break
		} else {
			i += 1
			fmt.Println()
		}
	}
}
