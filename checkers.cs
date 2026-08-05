// checkers.cs
using System;
using System.Collections.Generic;

class Checkers
{
    const int EMPTY = 0;
    const int WHITE = 1;
    const int BLACK = 2;

    int[,] board = new int[8, 8];
    int turn = WHITE;

    public Checkers()
    {
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
                if ((row + col) % 2 == 1)
                {
                    if (row < 3) board[row, col] = WHITE;
                    else if (row > 4) board[row, col] = BLACK;
                }
    }

    void PrintBoard()
    {
        Console.WriteLine("  a b c d e f g h");
        for (int row = 0; row < 8; row++)
        {
            Console.Write(row + 1 + " ");
            for (int col = 0; col < 8; col++)
            {
                char ch = '·';
                if (board[row, col] == WHITE) ch = '○';
                else if (board[row, col] == BLACK) ch = '●';
                Console.Write(ch + " ");
            }
            Console.WriteLine();
        }
    }

    bool IsValidPos(int row, int col) => row >= 0 && row < 8 && col >= 0 && col < 8;

    List<(int, int, int, int, int, int)> GetMoves(int row, int col) // from_r, from_c, to_r, to_c, cap_r, cap_c
    {
        var moves = new List<(int, int, int, int, int, int)>();
        int piece = board[row, col];
        if (piece == EMPTY) return moves;
        int dir = piece == WHITE ? 1 : -1;
        // простые ходы
        foreach (int dc in new int[] { -1, 1 })
        {
            int nr = row + dir, nc = col + dc;
            if (IsValidPos(nr, nc) && board[nr, nc] == EMPTY)
                moves.Add((row, col, nr, nc, -1, -1));
        }
        // захваты
        foreach (int dc in new int[] { -1, 1 })
        {
            int nr = row + dir * 2, nc = col + dc * 2;
            if (IsValidPos(nr, nc))
            {
                int mr = row + dir, mc = col + dc;
                if (IsValidPos(mr, mc) && board[mr, mc] != EMPTY && board[mr, mc] != piece && board[nr, nc] == EMPTY)
                    moves.Add((row, col, nr, nc, mr, mc));
            }
        }
        return moves;
    }

    List<(int, int, int, int, int, int)> GetAllMoves(int color)
    {
        var moves = new List<(int, int, int, int, int, int)>();
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
                if (board[row, col] == color)
                    moves.AddRange(GetMoves(row, col));
        return moves;
    }

    bool HasCaptures(int color)
    {
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
                if (board[row, col] == color)
                    foreach (var m in GetMoves(row, col))
                        if (m.Item5 != -1)
                            return true;
        return false;
    }

    void MakeMove(int fr, int fc, int tr, int tc, int cr, int cc)
    {
        board[tr, tc] = board[fr, fc];
        board[fr, fc] = EMPTY;
        if (cr != -1) board[cr, cc] = EMPTY;
    }

    bool IsGameOver()
    {
        var wm = GetAllMoves(WHITE);
        var bm = GetAllMoves(BLACK);
        if (wm.Count == 0 || bm.Count == 0) return true;
        bool hasWhite = false, hasBlack = false;
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
            {
                if (board[row, col] == WHITE) hasWhite = true;
                else if (board[row, col] == BLACK) hasBlack = true;
            }
        return !hasWhite || !hasBlack;
    }

    int GetWinner()
    {
        var wm = GetAllMoves(WHITE);
        var bm = GetAllMoves(BLACK);
        bool hasWhite = false, hasBlack = false;
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
            {
                if (board[row, col] == WHITE) hasWhite = true;
                else if (board[row, col] == BLACK) hasBlack = true;
            }
        if (!hasWhite || wm.Count == 0) return BLACK;
        if (!hasBlack || bm.Count == 0) return WHITE;
        return 0;
    }

    (int, int, int, int)? ParseMove(string s)
    {
        var parts = s.Split(' ', StringSplitOptions.RemoveEmptyEntries);
        if (parts.Length != 2) return null;
        if (parts[0].Length < 2 || parts[1].Length < 2) return null;
        int col1 = parts[0][0] - 'a';
        int row1 = parts[0][1] - '1';
        int col2 = parts[1][0] - 'a';
        int row2 = parts[1][1] - '1';
        if (row1 < 0 || row1 > 7 || col1 < 0 || col1 > 7 || row2 < 0 || row2 > 7 || col2 < 0 || col2 > 7)
            return null;
        return (row1, col1, row2, col2);
    }

    public void Play()
    {
        Console.WriteLine("Добро пожаловать в мини-шашки!");
        Console.WriteLine("Белые (○) ходят первыми.");
        Console.WriteLine("Вводите ход в формате: a2 b3");
        while (!IsGameOver())
        {
            PrintBoard();
            int color = turn;
            string colorName = color == WHITE ? "белых (○)" : "чёрных (●)";
            Console.WriteLine($"Ход {colorName}.");
            var allMoves = GetAllMoves(color);
            if (allMoves.Count == 0)
            {
                Console.WriteLine("Нет доступных ходов, игра окончена.");
                break;
            }
            bool hasCap = HasCaptures(color);
            var moves = allMoves;
            if (hasCap)
            {
                Console.WriteLine("Обязательный захват!");
                moves = allMoves.FindAll(m => m.Item5 != -1);
            }
            while (true)
            {
                Console.Write("> ");
                string cmd = Console.ReadLine();
                if (cmd == "quit") return;
                var parsed = ParseMove(cmd);
                if (!parsed.HasValue)
                {
                    Console.WriteLine("Неверный формат. Используйте: a2 b3");
                    continue;
                }
                var (fr, fc, tr, tc) = parsed.Value;
                bool found = false;
                foreach (var m in moves)
                {
                    if (m.Item1 == fr && m.Item2 == fc && m.Item3 == tr && m.Item4 == tc)
                    {
                        MakeMove(m.Item1, m.Item2, m.Item3, m.Item4, m.Item5, m.Item6);
                        found = true;
                        break;
                    }
                }
                if (!found)
                {
                    Console.WriteLine("Неверный ход. Попробуйте снова.");
                    continue;
                }
                turn = (color == WHITE) ? BLACK : WHITE;
                break;
            }
        }
        PrintBoard();
        int winner = GetWinner();
        if (winner == WHITE) Console.WriteLine("Победили белые (○)!");
        else if (winner == BLACK) Console.WriteLine("Победили чёрные (●)!");
        else Console.WriteLine("Ничья?");
    }

    static void Main()
    {
        Checkers game = new Checkers();
        game.Play();
    }
}
