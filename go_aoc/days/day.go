package days

type Day interface {
    GetFilename() string
    Solution1(input string) string
    Solution2(input string) string
}
