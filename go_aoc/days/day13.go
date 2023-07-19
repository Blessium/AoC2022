package days

import (
	"strconv"
	"strings"
)

type State uint

const (
	BothNumbers State = iota
	RightList
	LeftList
	BothList
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

func GetDiff(l *Signal, r *Signal) bool {
	maxLen := len(l.values)
	prevj := 0
	previ := 0
	for i, j := 0, 0; i < maxLen && j < maxLen; i, j = i+1, j+1 {
		v1 := l.values[i]
		v2 := r.values[j]
		switch evaluateState(v1, v2) {
		case BothList:
			{
                previ = i
                prevj = j
                l = l.next[i]
                r = r.next[j]
			}
		}
	}
	return true
}

func (d Day13) GetFilename() string {
	return "day13"
}

func (d Day13) Solution1(input string) string {
	line := strings.Split(input, "\n")[0]
	signal := NewSignal(line)
	return signal.String()
}

func (d Day13) Solution2(input string) string {
	return ""
}
