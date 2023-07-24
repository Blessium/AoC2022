package main

import (
	"fmt"
	"github.com/blessium/AoC2022/days"
	"io/ioutil"
)

func main() {
	day := days.Day13{}
	f := "./inputs/" + day.GetFilename()

	file_name := f + "_test.txt"
	content, err := getFileContent(file_name)
	if err != nil {
		panic(err.Error())
	}
	sol := day.Solution1(content)
	fmt.Printf("Solution test on solution 1: %s\n", sol)
	sol = day.Solution2(content)
	fmt.Printf("Solution test on solution 2: %s\n", sol)

	file_name = f + "_1.txt"
	content, err = getFileContent(file_name)
	if err != nil {
		panic(err.Error())
	}
	sol = day.Solution1(content)
	fmt.Printf("Solution 1: %s\n", sol)

	next := f + "_2.txt"
	elements := []string{file_name, next}
	for _, file := range elements {
		content, err = getFileContent(file)
		if err == nil {
			sol = day.Solution2(content)
			fmt.Printf("Solution 2: %s\n", sol)
			return
		}
	}
}

func getFileContent(f string) (string, error) {
	content, err := ioutil.ReadFile(f)
	if err != nil {
		return "", err
	}
	return string(content), nil
}
