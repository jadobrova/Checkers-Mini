// checkers.cpp
#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <cctype>
using namespace std;

const int EMPTY = 0;
const int WHITE = 1;
const int BLACK = 2;

struct Checkers {
    int board[8][8];
    int turn;

    Checkers() : turn(WHITE) {
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                board[r][c] = EMPTY;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if ((r + c) % 2 == 1) {
                    if (r < 3) board[r][c] = WHITE;
                    else if (r > 4) board[r][c] = BLACK;
                }
    }

    void printBoard() {
        cout << "  a b c d e f g h\n";
        for (int r = 0; r < 8; r++) {
            cout << r+1 << " ";
            for (int c = 0; c < 8; c++) {
                char ch = '·';
                if (board[r][c] == WHITE) ch = '○';
                else if (board[r][c] == BLACK) ch = '●';
                cout << ch << " ";
            }
            cout << "\n";
        }
    }

    bool isValidPos(int r, int c) {
        return r >= 0 && r < 8 && c >= 0 && c < 8;
    }

    vector<array<int,6>> getMoves(int r, int c) {
        vector<array<int,6>> moves; // from_r, from_c, to_r, to_c, cap_r, cap_c
        int piece = board[r][c];
        if (piece == EMPTY) return moves;
        int dir = piece == WHITE ? 1 : -1;
        // simple
        for (int dc : {-1, 1}) {
            int nr = r + dir, nc = c + dc;
            if (isValidPos(nr, nc) && board[nr][nc] == EMPTY)
                moves.push_back({r, c, nr, nc, -1, -1});
        }
        // captures
        for (int dc : {-1, 1}) {
            int nr = r + dir*2, nc = c + dc*2;
            if (isValidPos(nr, nc)) {
                int mr = r + dir, mc = c + dc;
                if (isValidPos(mr, mc) && board[mr][mc] != EMPTY && board[mr][mc] != piece && board[nr][nc] == EMPTY)
                    moves.push_back({r, c, nr, nc, mr, mc});
            }
        }
        return moves;
    }

    vector<array<int,6>> getAllMoves(int color) {
        vector<array<int,6>> all;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (board[r][c] == color) {
                    auto m = getMoves(r, c);
                    all.insert(all.end(), m.begin(), m.end());
                }
        return all;
    }

    bool hasCaptures(int color) {
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++)
                if (board[r][c] == color)
                    for (auto& m : getMoves(r, c))
                        if (m[4] != -1)
                            return true;
        return false;
    }

    void makeMove(array<int,6> m) {
        board[m[2]][m[3]] = board[m[0]][m[1]];
        board[m[0]][m[1]] = EMPTY;
        if (m[4] != -1) board[m[4]][m[5]] = EMPTY;
    }

    bool isGameOver() {
        auto wm = getAllMoves(WHITE);
        auto bm = getAllMoves(BLACK);
        if (wm.empty() || bm.empty()) return true;
        bool hasWhite = false, hasBlack = false;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++) {
                if (board[r][c] == WHITE) hasWhite = true;
                else if (board[r][c] == BLACK) hasBlack = true;
            }
        return !hasWhite || !hasBlack;
    }

    int getWinner() {
        auto wm = getAllMoves(WHITE);
        auto bm = getAllMoves(BLACK);
        bool hasWhite = false, hasBlack = false;
        for (int r = 0; r < 8; r++)
            for (int c = 0; c < 8; c++) {
                if (board[r][c] == WHITE) hasWhite = true;
                else if (board[r][c] == BLACK) hasBlack = true;
            }
        if (!hasWhite || wm.empty()) return BLACK;
        if (!hasBlack || bm.empty()) return WHITE;
        return 0;
    }

    array<int,4> parseMove(const string& s) {
        array<int,4> res = {-1,-1,-1,-1};
        stringstream ss(s);
        string from, to;
        ss >> from >> to;
        if (from.empty() || to.empty() || from.size()<2 || to.size()<2) return res;
        int c1 = from[0] - 'a';
        int r1 = from[1] - '1';
        int c2 = to[0] - 'a';
        int r2 = to[1] - '1';
        if (r1<0||r1>7||c1<0||c1>7||r2<0||r2>7||c2<0||c2>7) return res;
        res = {r1, c1, r2, c2};
        return res;
    }

    void play() {
        cout << "Добро пожаловать в мини-шашки!\n";
        cout << "Белые (○) ходят первыми.\n";
        cout << "Вводите ход в формате: a2 b3\n";
        string cmd;
        while (!isGameOver()) {
            printBoard();
            int color = turn;
            string colorName = (color == WHITE) ? "белых (○)" : "чёрных (●)";
            cout << "Ход " << colorName << ".\n";
            auto allMoves = getAllMoves(color);
            if (allMoves.empty()) {
                cout << "Нет доступных ходов, игра окончена.\n";
                break;
            }
            bool hasCap = hasCaptures(color);
            vector<array<int,6>> moves = allMoves;
            if (hasCap) {
                cout << "Обязательный захват!\n";
                moves.clear();
                for (auto& m : allMoves)
                    if (m[4] != -1) moves.push_back(m);
            }
            while (true) {
                cout << "> ";
                getline(cin, cmd);
                if (cmd == "quit") return;
                auto parsed = parseMove(cmd);
                if (parsed[0] == -1) {
                    cout << "Неверный формат. Используйте: a2 b3\n";
                    continue;
                }
                int fr=parsed[0], fc=parsed[1], tr=parsed[2], tc=parsed[3];
                bool found = false;
                for (auto& m : moves) {
                    if (m[0]==fr && m[1]==fc && m[2]==tr && m[3]==tc) {
                        makeMove(m);
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    cout << "Неверный ход. Попробуйте снова.\n";
                    continue;
                }
                turn = (color == WHITE) ? BLACK : WHITE;
                break;
            }
        }
        printBoard();
        int winner = getWinner();
        if (winner == WHITE) cout << "Победили белые (○)!\n";
        else if (winner == BLACK) cout << "Победили чёрные (●)!\n";
        else cout << "Ничья?\n";
    }
};

int main() {
    Checkers game;
    game.play();
    return 0;
}
