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

	for i := range cave {
		cave[i] = make([]rune, (dimension.Max.X + 2))
	}

	prev_coo := rocks[0].Structure[0]

	for _, rock := range rocks {
		prev_coo = rock.Structure[0]
		prev_coo.X -= dimension.Min.X
		for _, t := range rock.Structure {
			current := Coordinates{X: t.X - dimension.Min.X , Y: t.Y}
			diff_x := current.X - prev_coo.X
			diff_y := current.Y - prev_coo.Y
			x_neg := math.Signbit(float64(diff_x))
			y_neg := math.Signbit(float64(diff_y))

			for i := 0; i <= int(math.Abs(float64(diff_x))); i += 1 {
				if x_neg {
					cave[prev_coo.Y][prev_coo.X-i + 1] = '#'
				} else {
					cave[prev_coo.Y][prev_coo.X+i + 1] = '#'
				}
			}

			for i := 0; i < int(math.Abs(float64(diff_y))); i += 1 {
				if y_neg {
					cave[prev_coo.Y-i][prev_coo.X + 1] = '#'
				} else {
					cave[prev_coo.Y+i][prev_coo.X + 1] = '#'
				}
			}
			prev_coo = current
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

func (c Cave) StartSimulation(sand_start Coordinates) int {
	validMovements := []Coordinates{
		{X: 0, Y: 1},
		{X: -1, Y: 1},
		{X: 1, Y: 1},
	}
	counter := 0
	isFlowing := false

	for isFlowing == false {
		sand := sand_start
		for {
			isValid := false
			for _, mov := range validMovements {
				if len(c.Cave) <= sand.Y+mov.Y {
					isFlowing = true
					continue

				}
				temp := c.Cave[sand.Y+mov.Y][sand.X+mov.X]
				if temp == 0 {
					sand.Y += mov.Y
					sand.X += mov.X
					isFlowing = false
					isValid = true
					break
				}
			}

			if !isValid {
				c.Cave[sand.Y][sand.X] = 'O'
                if sand.X == sand_start.X && sand.Y == sand_start.Y {
                    return counter + 1
                }
				break
			}

		}
        if isFlowing == false {
		    counter += 1
        }
	}

	return counter
}

func (c Cave) String() {
	for i := range c.Cave {
		for j := range c.Cave[i] {
			if c.Cave[i][j] == '#' || c.Cave[i][j] == 'O' {
				fmt.Printf("%c", c.Cave[i][j])
			} else {
				fmt.Print(".")
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
    result := cave.StartSimulation(Coordinates{X: 501 - cave_size.Min.X, Y: cave_size.Min.Y})
	return strconv.Itoa(result)
}
func (d Day14) Solution2(input string) string {
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
    cave_size.Max.Y += 2
    cave_size.Min.X = int(cave_size.Max.X / 2)

    start_floor := Coordinates {X: cave_size.Min.X, Y: cave_size.Max.Y}
    end_floor := Coordinates {X: cave_size.Max.X + cave_size.Min.X, Y: cave_size.Max.Y}
    floor_coo := []Coordinates {start_floor, end_floor}
    floor := Rock {Structure: floor_coo}
    rocks = append(rocks, floor)
	cave := NewCave(rocks, cave_size)
    result := cave.StartSimulation(Coordinates{X: 501 - cave_size.Min.X, Y: cave_size.Min.Y})
	return strconv.Itoa(result)
}
