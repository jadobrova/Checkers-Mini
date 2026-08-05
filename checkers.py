# checkers.py
import sys

EMPTY = 0
WHITE = 1  # ходят первыми
BLACK = 2

class Checkers:
    def __init__(self):
        self.board = [[EMPTY]*8 for _ in range(8)]
        # расстановка: белые снизу (строки 0-2), чёрные сверху (строки 5-7)
        for row in range(3):
            for col in range(8):
                if (row + col) % 2 == 1:
                    self.board[row][col] = WHITE
        for row in range(5, 8):
            for col in range(8):
                if (row + col) % 2 == 1:
                    self.board[row][col] = BLACK
        self.turn = WHITE  # белые ходят первыми

    def print_board(self):
        print("  a b c d e f g h")
        for row in range(8):
            print(row+1, end=" ")
            for col in range(8):
                piece = self.board[row][col]
                if piece == EMPTY:
                    ch = '·'
                elif piece == WHITE:
                    ch = '○'
                else:
                    ch = '●'
                print(ch, end=" ")
            print()

    def is_valid_pos(self, row, col):
        return 0 <= row < 8 and 0 <= col < 8

    def get_piece(self, row, col):
        if self.is_valid_pos(row, col):
            return self.board[row][col]
        return None

    # Получить все ходы для шашки (без учёта обязательности)
    def get_moves(self, row, col):
        piece = self.board[row][col]
        if piece == EMPTY:
            return []
        direction = 1 if piece == WHITE else -1
        moves = []
        # ход на одну клетку
        for dc in (-1, 1):
            nr, nc = row + direction, col + dc
            if self.is_valid_pos(nr, nc) and self.board[nr][nc] == EMPTY:
                moves.append(((row, col), (nr, nc), None))
        # захват (прыжок)
        for dc in (-1, 1):
            nr, nc = row + direction*2, col + dc*2
            if self.is_valid_pos(nr, nc):
                mid_r, mid_c = row + direction, col + dc
                if self.is_valid_pos(mid_r, mid_c) and self.board[mid_r][mid_c] != EMPTY and self.board[mid_r][mid_c] != piece:
                    if self.board[nr][nc] == EMPTY:
                        moves.append(((row, col), (nr, nc), (mid_r, mid_c)))
        return moves

    def get_all_moves(self, color):
        moves = []
        for row in range(8):
            for col in range(8):
                if self.board[row][col] == color:
                    moves.extend(self.get_moves(row, col))
        return moves

    # Проверить, есть ли захваты
    def has_captures(self, color):
        for row in range(8):
            for col in range(8):
                if self.board[row][col] == color:
                    for move in self.get_moves(row, col):
                        if move[2] is not None:  # захват
                            return True
        return False

    def make_move(self, from_pos, to_pos, capture_pos=None):
        fr, fc = from_pos
        tr, tc = to_pos
        self.board[tr][tc] = self.board[fr][fc]
        self.board[fr][fc] = EMPTY
        if capture_pos:
            cr, cc = capture_pos
            self.board[cr][cc] = EMPTY

    def is_game_over(self):
        # если у одного из игроков нет шашек или нет ходов
        white_moves = self.get_all_moves(WHITE)
        black_moves = self.get_all_moves(BLACK)
        if not white_moves or not black_moves:
            return True
        # проверка на наличие шашек
        white_pieces = any(self.board[r][c] == WHITE for r in range(8) for c in range(8))
        black_pieces = any(self.board[r][c] == BLACK for r in range(8) for c in range(8))
        return not white_pieces or not black_pieces

    def get_winner(self):
        white_moves = self.get_all_moves(WHITE)
        black_moves = self.get_all_moves(BLACK)
        white_pieces = any(self.board[r][c] == WHITE for r in range(8) for c in range(8))
        black_pieces = any(self.board[r][c] == BLACK for r in range(8) for c in range(8))
        if not white_pieces or not white_moves:
            return BLACK
        if not black_pieces or not black_moves:
            return WHITE
        return None

    def parse_move(self, s):
        # ожидается "a2 b3"
        parts = s.strip().split()
        if len(parts) != 2:
            return None
        try:
            col1 = ord(parts[0][0]) - ord('a')
            row1 = int(parts[0][1]) - 1
            col2 = ord(parts[1][0]) - ord('a')
            row2 = int(parts[1][1]) - 1
            if not (0 <= row1 < 8 and 0 <= col1 < 8 and 0 <= row2 < 8 and 0 <= col2 < 8):
                return None
            return (row1, col1), (row2, col2)
        except:
            return None

    def play(self):
        print("Добро пожаловать в мини-шашки!")
        print("Белые (○) ходят первыми.")
        print("Вводите ход в формате: a2 b3")
        while not self.is_game_over():
            self.print_board()
            color = self.turn
            color_name = "белых (○)" if color == WHITE else "чёрных (●)"
            print(f"Ход {color_name}.")
            # Проверяем, есть ли обязательные захваты
            captures = self.has_captures(color)
            # Получаем все возможные ходы
            all_moves = self.get_all_moves(color)
            if not all_moves:
                print("Нет доступных ходов, игра окончена.")
                break
            # Фильтруем ходы, если есть захваты – только захваты
            if captures:
                moves = [m for m in all_moves if m[2] is not None]
                print("Обязательный захват!")
            else:
                moves = all_moves
            # Ввод от пользователя
            while True:
                cmd = input("> ").strip()
                if cmd.lower() == 'quit':
                    return
                parsed = self.parse_move(cmd)
                if parsed is None:
                    print("Неверный формат. Используйте: a2 b3")
                    continue
                from_pos, to_pos = parsed
                # Ищем ход в списке
                found = None
                for m in moves:
                    if m[0] == from_pos and m[1] == to_pos:
                        found = m
                        break
                if found is None:
                    print("Неверный ход. Попробуйте снова.")
                    continue
                # Выполняем ход
                self.make_move(found[0], found[1], found[2])
                self.turn = BLACK if color == WHITE else WHITE
                break
        # Конец игры
        self.print_board()
        winner = self.get_winner()
        if winner == WHITE:
            print("Победили белые (○)!")
        elif winner == BLACK:
            print("Победили чёрные (●)!")
        else:
            print("Ничья? (такого не должно быть)")

if __name__ == "__main__":
    game = Checkers()
    game.play()
