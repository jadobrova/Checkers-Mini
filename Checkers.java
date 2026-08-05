// Checkers.java
import java.util.*;

public class Checkers {
    static final int EMPTY = 0;
    static final int WHITE = 1;
    static final int BLACK = 2;

    int[][] board = new int[8][8];
    int turn = WHITE;
    Scanner scanner = new Scanner(System.in);

    public Checkers() {
        for (int row = 0; row < 8; row++) {
            for (int col = 0; col < 8; col++) {
                if ((row + col) % 2 == 1) {
                    if (row < 3) board[row][col] = WHITE;
                    else if (row > 4) board[row][col] = BLACK;
                }
            }
        }
    }

    void printBoard() {
        System.out.println("  a b c d e f g h");
        for (int row = 0; row < 8; row++) {
            System.out.print((row + 1) + " ");
            for (int col = 0; col < 8; col++) {
                char ch = '·';
                if (board[row][col] == WHITE) ch = '○';
                else if (board[row][col] == BLACK) ch = '●';
                System.out.print(ch + " ");
            }
            System.out.println();
        }
    }

    boolean isValidPos(int row, int col) {
        return row >= 0 && row < 8 && col >= 0 && col < 8;
    }

    List<int[]> getMoves(int row, int col) {
        // returns arrays: {from_r, from_c, to_r, to_c, cap_r, cap_c}
        List<int[]> moves = new ArrayList<>();
        int piece = board[row][col];
        if (piece == EMPTY) return moves;
        int dir = piece == WHITE ? 1 : -1;
        // simple moves
        for (int dc : new int[]{-1, 1}) {
            int nr = row + dir, nc = col + dc;
            if (isValidPos(nr, nc) && board[nr][nc] == EMPTY)
                moves.add(new int[]{row, col, nr, nc, -1, -1});
        }
        // captures
        for (int dc : new int[]{-1, 1}) {
            int nr = row + dir*2, nc = col + dc*2;
            if (isValidPos(nr, nc)) {
                int mr = row + dir, mc = col + dc;
                if (isValidPos(mr, mc) && board[mr][mc] != EMPTY && board[mr][mc] != piece && board[nr][nc] == EMPTY)
                    moves.add(new int[]{row, col, nr, nc, mr, mc});
            }
        }
        return moves;
    }

    List<int[]> getAllMoves(int color) {
        List<int[]> moves = new ArrayList<>();
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
                if (board[row][col] == color)
                    moves.addAll(getMoves(row, col));
        return moves;
    }

    boolean hasCaptures(int color) {
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++)
                if (board[row][col] == color)
                    for (int[] m : getMoves(row, col))
                        if (m[4] != -1)
                            return true;
        return false;
    }

    void makeMove(int fr, int fc, int tr, int tc, int cr, int cc) {
        board[tr][tc] = board[fr][fc];
        board[fr][fc] = EMPTY;
        if (cr != -1) board[cr][cc] = EMPTY;
    }

    boolean isGameOver() {
        List<int[]> wm = getAllMoves(WHITE);
        List<int[]> bm = getAllMoves(BLACK);
        if (wm.isEmpty() || bm.isEmpty()) return true;
        boolean hasWhite = false, hasBlack = false;
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++) {
                if (board[row][col] == WHITE) hasWhite = true;
                else if (board[row][col] == BLACK) hasBlack = true;
            }
        return !hasWhite || !hasBlack;
    }

    int getWinner() {
        List<int[]> wm = getAllMoves(WHITE);
        List<int[]> bm = getAllMoves(BLACK);
        boolean hasWhite = false, hasBlack = false;
        for (int row = 0; row < 8; row++)
            for (int col = 0; col < 8; col++) {
                if (board[row][col] == WHITE) hasWhite = true;
                else if (board[row][col] == BLACK) hasBlack = true;
            }
        if (!hasWhite || wm.isEmpty()) return BLACK;
        if (!hasBlack || bm.isEmpty()) return WHITE;
        return 0;
    }

    int[] parseMove(String s) {
        String[] parts = s.trim().split("\\s+");
        if (parts.length != 2) return null;
        if (parts[0].length() < 2 || parts[1].length() < 2) return null;
        int col1 = parts[0].charAt(0) - 'a';
        int row1 = parts[0].charAt(1) - '1';
        int col2 = parts[1].charAt(0) - 'a';
        int row2 = parts[1].charAt(1) - '1';
        if (row1 < 0 || row1 > 7 || col1 < 0 || col1 > 7 || row2 < 0 || row2 > 7 || col2 < 0 || col2 > 7)
            return null;
        return new int[]{row1, col1, row2, col2};
    }

    void play() {
        System.out.println("Добро пожаловать в мини-шашки!");
        System.out.println("Белые (○) ходят первыми.");
        System.out.println("Вводите ход в формате: a2 b3");
        while (!isGameOver()) {
            printBoard();
            int color = turn;
            String colorName = (color == WHITE) ? "белых (○)" : "чёрных (●)";
            System.out.println("Ход " + colorName + ".");
            List<int[]> allMoves = getAllMoves(color);
            if (allMoves.isEmpty()) {
                System.out.println("Нет доступных ходов, игра окончена.");
                break;
            }
            boolean hasCap = hasCaptures(color);
            List<int[]> moves = allMoves;
            if (hasCap) {
                System.out.println("Обязательный захват!");
                moves = new ArrayList<>();
                for (int[] m : allMoves)
                    if (m[4] != -1) moves.add(m);
            }
            while (true) {
                System.out.print("> ");
                String cmd = scanner.nextLine().trim();
                if (cmd.equals("quit")) return;
                int[] parsed = parseMove(cmd);
                if (parsed == null) {
                    System.out.println("Неверный формат. Используйте: a2 b3");
                    continue;
                }
                int fr = parsed[0], fc = parsed[1], tr = parsed[2], tc = parsed[3];
                boolean found = false;
                for (int[] m : moves) {
                    if (m[0] == fr && m[1] == fc && m[2] == tr && m[3] == tc) {
                        makeMove(m[0], m[1], m[2], m[3], m[4], m[5]);
                        found = true;
                        break;
                    }
                }
                if (!found) {
                    System.out.println("Неверный ход. Попробуйте снова.");
                    continue;
                }
                turn = (color == WHITE) ? BLACK : WHITE;
                break;
            }
        }
        printBoard();
        int winner = getWinner();
        if (winner == WHITE) System.out.println("Победили белые (○)!");
        else if (winner == BLACK) System.out.println("Победили чёрные (●)!");
        else System.out.println("Ничья?");
    }

    public static void main(String[] args) {
        Checkers game = new Checkers();
        game.play();
    }
}
