🏆 Checkers Mini – Классические шашки в консоли
Минималистичная реализация игры в шашки для двух игроков с понятным интерфейсом и проверкой правил.
Реализована на 7 языках программирования – выберите свой!

✨ Особенности
♟️ Полноценная доска 8×8 – стандартная расстановка шашек.

🎯 Простые правила – ходы по диагонали вперёд, захват прыжком (обязательный бой).

🔄 Пошаговая игра – два игрока ходят по очереди (белые и чёрные).

⌨️ Удобный ввод – введите координаты в формате a2 b3 (откуда → куда).

🏆 Определение победителя – игра завершается, когда у одного игрока не остаётся шашек или ходов.

🖥️ Кроссплатформенность – работает в Linux, macOS и Windows (везде, где есть терминал).

🎮 Управление
Игроки по очереди вводят ход в формате a2 b3 (буква столбца + номер строки).

Доска отображается с буквами по горизонтали (a–h) и цифрами по вертикали (1–8).

Белые шашки обозначаются ○, чёрные – ● (или X/O в зависимости от реализации).

При захвате шашка соперника удаляется с доски.

Если есть возможность захвата, она обязательна (в мини-версии).

📦 Поддерживаемые языки
Язык	Файл	Запуск
Python	checkers.py	python checkers.py
Go	checkers.go	go run checkers.go
Rust	checkers.rs	cargo run
JavaScript	checkers.js	node checkers.js
C#	checkers.cs	dotnet run
Java	Checkers.java	javac Checkers.java && java Checkers
C++	checkers.cpp	g++ checkers.cpp -o checkers && ./checkers
🚀 Быстрый старт
1. Склонируйте репозиторий
bash
git clone https://github.com/yourname/checkers-mini.git
cd checkers-mini
2. Запустите на любом языке
Python

bash
python checkers.py
Go

bash
go run checkers.go
Rust (сборка)

bash
cargo new checkers --bin
# скопируйте код в src/main.rs
cargo run
JavaScript (Node.js)

bash
node checkers.js
C#

bash
dotnet new console -n checkers
# скопируйте код в Program.cs
dotnet run
Java

bash
javac Checkers.java
java Checkers
C++

bash
g++ -std=c++17 checkers.cpp -o checkers
./checkers
📋 Пример игровой сессии
text
  a b c d e f g h
1 · · · · · · · ·
2 · ○ · ○ · ○ · ○
3 ○ · ○ · ○ · ○ ·
4 · · · · · · · ·
5 · · · · · · · ·
6 · ● · ● · ● · ●
7 ● · ● · ● · ● ·
8 · · · · · · · ·

Ход белых (○). Введите ход (например, a2 b3): a2 b3
...
📄 Лицензия
MIT – свободно используйте, модифицируйте и распространяйте.

🤝 Вклад
Приветствуются pull request'ы! Если хотите добавить новый язык или улучшить существующий – создавайте issue.

🧠 Авторы
Проект создан в образовательных целях для демонстрации игровой логики и алгоритмов на разных языках.

