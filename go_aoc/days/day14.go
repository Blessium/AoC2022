package days

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day14 struct {
}

type Coordinates struct {
	X, Y int
}

type Rock struct {
	Structure []Coordinates
}

type Cave struct {
	Cave [][]rune
}

func NewCave(rocks []Rock, dimension *CaveDimension) Cave {

	cave := make([][]rune, (dimension.Max.Y + 1))
	fmt.Printf("Dimension is: %v\n", dimension)
	for i := range cave {
		cave[i] = make([]rune, (dimension.Max.X - dimension.Min.X + 1))
	}

	prev_coo := rocks[0].Structure[0]

	for _, rock := range rocks {
		prev_coo = rock.Structure[0]
		prev_coo.X -= dimension.Min.X
		for _, t := range rock.Structure {
			current := Coordinates{X: t.X - dimension.Min.X, Y: t.Y}
			diff_x := current.X - prev_coo.X
			diff_y := current.Y - prev_coo.Y
			x_neg := math.Signbit(float64(diff_x))
			y_neg := math.Signbit(float64(diff_y))
			fmt.Printf("FROM(Y:%d, X:%d)\n", prev_coo.Y, prev_coo.X)
			fmt.Printf("DIFF(Y:%d, X:%d)\n", diff_y, diff_x)
			fmt.Printf("  TO(Y:%d, X:%d)\n", current.Y, current.X)

			for i := 0; i < int(math.Abs(float64(diff_x))); i += 1 {
				if x_neg {
					cave[prev_coo.Y][prev_coo.X-i] = '#'
				} else {
					cave[prev_coo.Y][prev_coo.X+i] = '#'
				}
			}

			for i := 0; i < int(math.Abs(float64(diff_y))); i += 1 {
				if y_neg {
					cave[prev_coo.Y-i][prev_coo.X] = '#'
				} else {
					cave[prev_coo.Y+i][prev_coo.X] = '#'
				}
			}
			prev_coo = current
			fmt.Println()
		}
	}

	return Cave{
		Cave: cave,
	}
}

func ParseCordinates(line string) []Coordinates {
	c := strings.Split(line, " -> ")
	var result []Coordinates
	for _, n := range c {
		coordinate := strings.Split(n, ",")
		x, err := strconv.Atoi(coordinate[0])
		if err != nil {
			panic(err.Error())
		}
		y, err := strconv.Atoi(coordinate[1])
		if err != nil {
			panic(err.Error())
		}

		temp := Coordinates{X: x, Y: y}
		result = append(result, temp)
	}
	return result
}

func (c Cave) StartSimulation() int {
	return 0
}

func (c Cave) String() {
	for i := range c.Cave {
		for j := range c.Cave[i] {
			if c.Cave[i][j] == '#' {
				fmt.Printf("%c", c.Cave[i][j])
			} else {
                fmt.Print("@")
            }
		}
		fmt.Println("")
	}
}

type CaveDimension struct {
	Max Coordinates
	Min Coordinates
}

func (c *CaveDimension) Update(n Coordinates) {
	if c.Max.X < n.X {
		c.Max.X = n.X
	}
	if c.Max.Y < n.Y {
		c.Max.Y = n.Y
	}
	if c.Min.X > n.X {
		c.Min.X = n.X
	}
	if c.Min.Y > n.Y {
		c.Min.Y = n.Y
	}
}

func (d Day14) GetFilename() string {
	return "day14"
}

func (d Day14) Solution1(input string) string {
	cave_size := &CaveDimension{
		Max: Coordinates{
			X: math.MinInt,
			Y: math.MinInt,
		},
		Min: Coordinates{
			X: math.MaxInt,
			Y: math.MaxInt,
		},
	}

	var rocks []Rock

	lines := strings.Split(input, "\n")
	lines = lines[:len(lines)-1]

	for _, line := range lines {
		coordinates := ParseCordinates(line)
		for _, coordinate := range coordinates {
			cave_size.Update(coordinate)
		}
		rock := Rock{
			Structure: coordinates,
		}
		rocks = append(rocks, rock)
	}

    cave_size.Min.Y = 0
	cave := NewCave(rocks, cave_size)

	cave.String()

	return ""
}
func (d Day14) Solution2(input string) string {
	return ""
}
