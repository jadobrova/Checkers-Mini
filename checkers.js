// checkers.js
const readline = require('readline');

const EMPTY = 0;
const WHITE = 1;
const BLACK = 2;

class Checkers {
    constructor() {
        this.board = Array.from({ length: 8 }, () => Array(8).fill(EMPTY));
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                if ((row + col) % 2 === 1) {
                    if (row < 3) this.board[row][col] = WHITE;
                    else if (row > 4) this.board[row][col] = BLACK;
                }
            }
        }
        this.turn = WHITE;
        this.rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
    }

    printBoard() {
        console.log('  a b c d e f g h');
        for (let row = 0; row < 8; row++) {
            let line = (row + 1) + ' ';
            for (let col = 0; col < 8; col++) {
                const ch = this.board[row][col] === EMPTY ? '·' :
                           this.board[row][col] === WHITE ? '○' : '●';
                line += ch + ' ';
            }
            console.log(line);
        }
    }

    isValidPos(row, col) {
        return row >= 0 && row < 8 && col >= 0 && col < 8;
    }

    getMoves(row, col) {
        const piece = this.board[row][col];
        if (piece === EMPTY) return [];
        const dir = piece === WHITE ? 1 : -1;
        const moves = [];
        // простые ходы
        for (const dc of [-1, 1]) {
            const nr = row + dir, nc = col + dc;
            if (this.isValidPos(nr, nc) && this.board[nr][nc] === EMPTY) {
                moves.push({ from: [row, col], to: [nr, nc], capture: null });
            }
        }
        // захваты
        for (const dc of [-1, 1]) {
            const nr = row + dir*2, nc = col + dc*2;
            if (this.isValidPos(nr, nc)) {
                const mr = row + dir, mc = col + dc;
                if (this.isValidPos(mr, mc) && this.board[mr][mc] !== EMPTY && this.board[mr][mc] !== piece && this.board[nr][nc] === EMPTY) {
                    moves.push({ from: [row, col], to: [nr, nc], capture: [mr, mc] });
                }
            }
        }
        return moves;
    }

    getAllMoves(color) {
        let moves = [];
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                if (this.board[row][col] === color) {
                    moves = moves.concat(this.getMoves(row, col));
                }
            }
        }
        return moves;
    }

    hasCaptures(color) {
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                if (this.board[row][col] === color) {
                    for (const m of this.getMoves(row, col)) {
                        if (m.capture) return true;
                    }
                }
            }
        }
        return false;
    }

    makeMove(move) {
        const [fr, fc] = move.from;
        const [tr, tc] = move.to;
        this.board[tr][tc] = this.board[fr][fc];
        this.board[fr][fc] = EMPTY;
        if (move.capture) {
            const [cr, cc] = move.capture;
            this.board[cr][cc] = EMPTY;
        }
    }

    isGameOver() {
        const whiteMoves = this.getAllMoves(WHITE);
        const blackMoves = this.getAllMoves(BLACK);
        if (whiteMoves.length === 0 || blackMoves.length === 0) return true;
        let hasWhite = false, hasBlack = false;
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                if (this.board[row][col] === WHITE) hasWhite = true;
                else if (this.board[row][col] === BLACK) hasBlack = true;
            }
        }
        return !hasWhite || !hasBlack;
    }

    getWinner() {
        const whiteMoves = this.getAllMoves(WHITE);
        const blackMoves = this.getAllMoves(BLACK);
        let hasWhite = false, hasBlack = false;
        for (let row = 0; row < 8; row++) {
            for (let col = 0; col < 8; col++) {
                if (this.board[row][col] === WHITE) hasWhite = true;
                else if (this.board[row][col] === BLACK) hasBlack = true;
            }
        }
        if (!hasWhite || whiteMoves.length === 0) return BLACK;
        if (!hasBlack || blackMoves.length === 0) return WHITE;
        return 0;
    }

    parseMove(s) {
        const parts = s.trim().split(/\s+/);
        if (parts.length !== 2) return null;
        if (parts[0].length < 2 || parts[1].length < 2) return null;
        const col1 = parts[0].charCodeAt(0) - 97;
        const row1 = parseInt(parts[0][1]) - 1;
        const col2 = parts[1].charCodeAt(0) - 97;
        const row2 = parseInt(parts[1][1]) - 1;
        if (row1 < 0 || row1 > 7 || col1 < 0 || col1 > 7 || row2 < 0 || row2 > 7 || col2 < 0 || col2 > 7) return null;
        return { from: [row1, col1], to: [row2, col2] };
    }

    play() {
        console.log('Добро пожаловать в мини-шашки!');
        console.log('Белые (○) ходят первыми.');
        console.log('Вводите ход в формате: a2 b3');
        const prompt = () => {
            if (this.isGameOver()) {
                this.printBoard();
                const winner = this.getWinner();
                if (winner === WHITE) console.log('Победили белые (○)!');
                else if (winner === BLACK) console.log('Победили чёрные (●)!');
                else console.log('Ничья?');
                this.rl.close();
                return;
            }
            this.printBoard();
            const color = this.turn;
            const colorName = color === WHITE ? 'белых (○)' : 'чёрных (●)';
            console.log(`Ход ${colorName}.`);
            const allMoves = this.getAllMoves(color);
            if (allMoves.length === 0) {
                console.log('Нет доступных ходов, игра окончена.');
                this.rl.close();
                return;
            }
            const hasCap = this.hasCaptures(color);
            let moves = allMoves;
            if (hasCap) {
                console.log('Обязательный захват!');
                moves = allMoves.filter(m => m.capture !== null);
            }
            this.rl.question('> ', (cmd) => {
                if (cmd.trim() === 'quit') { this.rl.close(); return; }
                const parsed = this.parseMove(cmd);
                if (!parsed) {
                    console.log('Неверный формат. Используйте: a2 b3');
                    prompt();
                    return;
                }
                const found = moves.find(m => m.from[0] === parsed.from[0] && m.from[1] === parsed.from[1] &&
                                           m.to[0] === parsed.to[0] && m.to[1] === parsed.to[1]);
                if (!found) {
                    console.log('Неверный ход. Попробуйте снова.');
                    prompt();
                    return;
                }
                this.makeMove(found);
                this.turn = color === WHITE ? BLACK : WHITE;
                prompt();
            });
        };
        prompt();
    }
}

const game = new Checkers();
game.play();
