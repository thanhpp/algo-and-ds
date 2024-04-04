package gosol

import "strings"

func interpret(command string) string {
	var b = new(strings.Builder)

	for i := 0; i < len(command); {
		switch command[i] {
		case 'G':
			b.WriteString("G")
			i += 1
		case '(':
			if command[i+1] == ')' {
				b.WriteString("o")
				i += 2
				continue
			}
			b.WriteString("al")
			i += 4
		}
	}

	return b.String()
}
