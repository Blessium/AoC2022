package days

import (
	"strconv"
	"strings"
)

type State uint
type Result uint

const (
	BothNumbers State = iota
	RightList
	LeftList
	BothList
)

const (
    Correct Result = iota
    Continue
    Wrong
)

type Day13 struct {
}

type Signal struct {
	values []int
	prev   *Signal
	next   map[int]*Signal
}

func (s *Signal) String() string {
	buffer := "["
	for i, value := range s.values {
		if value == -1 {
			buffer += s.next[i].String()
		} else {
			buffer += string(strconv.Itoa(value))
		}
		if len(s.values)-1 == i {
			break
		}
		buffer += ","
	}
	buffer += "]"
	return buffer
}

func isDigit(ch rune) bool {
	return ch >= '0' && ch <= '9'
}

func isLPar(ch rune) bool {
	return ch == '['
}

func isRPar(ch rune) bool {
	return ch == ']'
}

func isComma(ch rune) bool {
	return ch == ','
}

func newEmptySignal() Signal {
	return Signal{
		prev: nil,
		next: make(map[int]*Signal),
	}
}

func NewSignal(input string) *Signal {
	start := newEmptySignal()
	current := &start
	input = input[1:]
	buffer_n := ""
	for i, ch := range input {
		switch {
		case isDigit(ch):
			{
				if isDigit(rune(input[i+1])) {
					buffer_n += string(ch)
					break
				}
				buffer_n += string(ch)
				n, err := strconv.Atoi(buffer_n)
				if err != nil {
					panic("What the fuck is this number")
				}
				current.values = append(current.values, n)
				buffer_n = ""
				break
			}
		case isLPar(ch):
			{
				current.values = append(current.values, -1)
				indexNext := len(current.values) - 1
				next := newEmptySignal()
				next.prev = current
				current.next[indexNext] = &next
				current = current.next[indexNext]
				break
			}
		case isRPar(ch):
			{
				current = current.prev
				break
			}

		case isComma(ch):
			{
				break
			}
		default:
			{
				panic("What the fuck is that char")
			}
		}
	}
	return &start
}

func evaluateState(l int, r int) State {
	if l == -1 {
		if r == -1 {
			return BothList
		} else {
			return LeftList
		}
	} else if r == -1 {
		return RightList
	}
	return BothNumbers
}

func GetDiff(l *Signal, r *Signal) Result {

	l_left := 0
    r_left := 0

	for i, j := 0, 0; i < len(l.values) && j < len(r.values); i, j = i+1, j+1 {
		switch evaluateState(l.values[i], r.values[j]) {
		case BothList:
			{
                result := GetDiff(l.next[i], r.next[j]) 
				if result  != Continue {
					return result
				}
			}
		case RightList:
			{
				values := []int{l.values[i]}
				signal := &Signal{
					values: values,
				}
                result := GetDiff(signal, r.next[j]) 
				if result  != Continue {
					return result
				}
			}

		case LeftList:
			{
				values := []int{r.values[j]}
				signal := &Signal{
					values: values,
				}
                result := GetDiff(l.next[i], signal)
				if result  != Continue {
					return result
				}
			}
		case BothNumbers:
			{
				if l.values[i] < r.values[j] {
					return Correct
				} else if l.values[i] > r.values[j] {
                    return Wrong
                }
			}
		}

        l_left = i
        r_left = j
	}

    diff_l := len(l.values) - l_left
    diff_r := len(r.values) - r_left
    if diff_l > diff_r {
        return Wrong
    } else if diff_r > diff_l {
        return Correct 
    } else {
        return Continue
    }
}

func (d Day13) GetFilename() string {
	return "day13"
}

func (d Day13) Solution1(input string) string {
    values := strings.Split(input, "\n")
    index := 1
    result := 0
    for i := 0; i < len(values); i += 3 {
        line := strings.Split(input, "\n")[i]
        line_2 := strings.Split(input, "\n")[i+1]
        signal := NewSignal(line)
        signal_2 := NewSignal(line_2)
        if GetDiff(signal, signal_2) == Correct {
            result += index
        }
        index += 1
    }
    return strconv.Itoa(result)
}

func (d Day13) Solution2(input string) string {
	return ""
}
