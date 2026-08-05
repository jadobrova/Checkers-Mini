// checkers.go
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const EMPTY = 0
const WHITE = 1
const BLACK = 2

type Checkers struct {
	board [8][8]int
	turn  int
}

func NewCheckers() *Checkers {
	c := &Checkers{turn: WHITE}
	// расстановка
	for row := 0; row < 8; row++ {
		for col := 0; col < 8; col++ {
			if (row+col)%2 == 1 {
				if row < 3 {
					c.board[row][col] = WHITE
				} else if row > 4 {
					c.board[row][col] = BLACK
				}
			}
		}
	}
	return c
}

func (c *Checkers) PrintBoard() {
	fmt.Println("  a b c d e f g h")
	for row := 0; row < 8; row++ {
		fmt.Printf("%d ", row+1)
		for col := 0; col < 8; col++ {
			ch := '·'
			if c.board[row][col] == WHITE {
				ch = '○'
			} else if c.board[row][col] == BLACK {
				ch = '●'
			}
			fmt.Printf("%c ", ch)
		}
		fmt.Println()
	}
}

func (c *Checkers) isValidPos(row, col int) bool {
	return row >= 0 && row < 8 && col >= 0 && col < 8
}

func (c *Checkers) getMoves(row, col int) [][3][2]int {
	piece := c.board[row][col]
	if piece == EMPTY {
		return nil
	}
	dir := 1
	if piece == BLACK {
		dir = -1
	}
	var moves [][3][2]int
	// простые ходы
	for _, dc := range []int{-1, 1} {
		nr, nc := row+dir, col+dc
		if c.isValidPos(nr, nc) && c.board[nr][nc] == EMPTY {
			moves = append(moves, [3][2]int{{row, col}, {nr, nc}, {-1, -1}})
		}
	}
	// захваты
	for _, dc := range []int{-1, 1} {
		nr, nc := row+dir*2, col+dc*2
		if c.isValidPos(nr, nc) {
			mr, mc := row+dir, col+dc
			if c.isValidPos(mr, mc) && c.board[mr][mc] != EMPTY && c.board[mr][mc] != piece && c.board[nr][nc] == EMPTY {
				moves = append(moves, [3][2]int{{row, col}, {nr, nc}, {mr, mc}})
			}
		}
	}
	return moves
}

func (c *Checkers) getAllMoves(color int) [][3][2]int {
	var moves [][3][2]int
	for row := 0; row < 8; row++ {
		for col := 0; col < 8; col++ {
			if c.board[row][col] == color {
				moves = append(moves, c.getMoves(row, col)...)
			}
		}
	}
	return moves
}

func (c *Checkers) hasCaptures(color int) bool {
	for row := 0; row < 8; row++ {
		for col := 0; col < 8; col++ {
			if c.board[row][col] == color {
				for _, m := range c.getMoves(row, col) {
					if m[2][0] != -1 {
						return true
					}
				}
			}
		}
	}
	return false
}

func (c *Checkers) makeMove(from [2]int, to [2]int, capture [2]int) {
	c.board[to[0]][to[1]] = c.board[from[0]][from[1]]
	c.board[from[0]][from[1]] = EMPTY
	if capture[0] != -1 {
		c.board[capture[0]][capture[1]] = EMPTY
	}
}

func (c *Checkers) isGameOver() bool {
	whiteMoves := c.getAllMoves(WHITE)
	blackMoves := c.getAllMoves(BLACK)
	if len(whiteMoves) == 0 || len(blackMoves) == 0 {
		return true
	}
	hasWhite := false
	hasBlack := false
	for row := 0; row < 8; row++ {
		for col := 0; col < 8; col++ {
			if c.board[row][col] == WHITE {
				hasWhite = true
			} else if c.board[row][col] == BLACK {
				hasBlack = true
			}
		}
	}
	return !hasWhite || !hasBlack
}

func (c *Checkers) getWinner() int {
	whiteMoves := c.getAllMoves(WHITE)
	blackMoves := c.getAllMoves(BLACK)
	hasWhite := false
	hasBlack := false
	for row := 0; row < 8; row++ {
		for col := 0; col < 8; col++ {
			if c.board[row][col] == WHITE {
				hasWhite = true
			} else if c.board[row][col] == BLACK {
				hasBlack = true
			}
		}
	}
	if !hasWhite || len(whiteMoves) == 0 {
		return BLACK
	}
	if !hasBlack || len(blackMoves) == 0 {
		return WHITE
	}
	return 0
}

func (c *Checkers) parseMove(s string) ([2]int, [2]int, bool) {
	parts := strings.Fields(s)
	if len(parts) != 2 {
		return [2]int{}, [2]int{}, false
	}
	if len(parts[0]) < 2 || len(parts[1]) < 2 {
		return [2]int{}, [2]int{}, false
	}
	col1 := int(parts[0][0] - 'a')
	row1 := int(parts[0][1] - '1')
	col2 := int(parts[1][0] - 'a')
	row2 := int(parts[1][1] - '1')
	if row1 < 0 || row1 > 7 || col1 < 0 || col1 > 7 || row2 < 0 || row2 > 7 || col2 < 0 || col2 > 7 {
		return [2]int{}, [2]int{}, false
	}
	return [2]int{row1, col1}, [2]int{row2, col2}, true
}

func (c *Checkers) Play() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Добро пожаловать в мини-шашки!")
	fmt.Println("Белые (○) ходят первыми.")
	fmt.Println("Вводите ход в формате: a2 b3")
	for !c.isGameOver() {
		c.PrintBoard()
		color := c.turn
		colorName := "белых (○)"
		if color == BLACK {
			colorName = "чёрных (●)"
		}
		fmt.Printf("Ход %s.\n", colorName)
		allMoves := c.getAllMoves(color)
		if len(allMoves) == 0 {
			fmt.Println("Нет доступных ходов, игра окончена.")
			break
		}
		hasCap := c.hasCaptures(color)
		moves := allMoves
		if hasCap {
			fmt.Println("Обязательный захват!")
			moves = nil
			for _, m := range allMoves {
				if m[2][0] != -1 {
					moves = append(moves, m)
				}
			}
		}
		for {
			fmt.Print("> ")
			cmd, _ := reader.ReadString('\n')
			cmd = strings.TrimSpace(cmd)
			if cmd == "quit" {
				return
			}
			from, to, ok := c.parseMove(cmd)
			if !ok {
				fmt.Println("Неверный формат. Используйте: a2 b3")
				continue
			}
			found := -1
			for i, m := range moves {
				if m[0] == from && m[1] == to {
					found = i
					break
				}
			}
			if found == -1 {
				fmt.Println("Неверный ход. Попробуйте снова.")
				continue
			}
			c.makeMove(moves[found][0], moves[found][1], moves[found][2])
			c.turn = BLACK
			if color == BLACK {
				c.turn = WHITE
			}
			break
		}
	}
	c.PrintBoard()
	winner := c.getWinner()
	if winner == WHITE {
		fmt.Println("Победили белые (○)!")
	} else if winner == BLACK {
		fmt.Println("Победили чёрные (●)!")
	} else {
		fmt.Println("Ничья?")
	}
}

func main() {
	game := NewCheckers()
	game.Play()
}
