# ltop

[![Rust](https://img.shields.io/badge/Rust-1.79%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> A lightweight terminal process manager written in Rust. No flickering, no jumping PIDs -- just a stable list and instant kill.

---

## Why

`htop` and `nvim` constantly refresh the process list. The line numbers jump around, making it impossible to track a PID and kill a frozen game or runaway app. **ltop** solves this with two simple modes: a live monitor and a static interactive killer.

---

## Features

- **Live Monitoring** -- auto-updating list of top memory-consuming processes.
- **Interactive Kill Mode** -- static list with user-assigned numbers. Enter a number, press Enter, process is gone.
- **Color-coded RSS** -- green (< 25 MB), yellow (< 1 GB), red (> 1 GB).
- **Kernel thread filtering** -- kworkers and system threads are hidden by default.
- **Zero dependencies** for core logic (only `console` for colors).

---

## Installation

```bash
git clone https://github.com/DuvalShapli/ltop.git
cd ltop
./install.sh
```

Optional system-wide install:
```bash
sudo cp target/release/ltop /usr/local/bin/
```

---

## Usage

### Monitoring mode (default)

```bash
ltop
# or
ltop 10
```

Shows the top N processes by RSS. Updates every time you press `R` (refresh) or `Q` (quit).

### Kill mode

```bash
ltop -k
ltop -k 10
```

Displays a static numbered list. Type the number of the process you want to kill and press Enter.

- `0` -- refresh the list.
- `1..N` -- send `SIGKILL` to the selected PID.

---

## Architecture

```
src/
|-- main.rs   # CLI args, mode selection, main loops
|-- proc.rs   # /proc parser, ProcessInfo struct, kernel-thread filter
|-- ui.rs     # Terminal output, colors, input helpers
```

---

## Tech Stack

- **Language:** Rust
- **Colors / terminal:** `console` crate

---

## License

MIT. See [LICENSE](LICENSE).

---

---

# ltop (RU)

> Легковесный менеджер процессов в терминале, написанный на Rust. Без мерцания, без прыгающих PID -- только стабильный список и мгновенное убийство процесса.

## Зачем

В `htop` и `nvim` список процессов постоянно обновляется, номера строк прыгают, и невозможно отследить PID, чтобы убить зависшую игру или съедающее память приложение. **ltop** решает это двумя режимами: живой мониторинг и статичный интерактивный киллер.

## Возможности

- **Мониторинг** -- автообновляемый список процессов по потреблению памяти.
- **Режим убийства** -- статичный список с номерами. Ввел номер, нажал Enter, процесс убит.
- **Цветовая индикация RSS** -- зеленый (< 25 МБ), желтый (< 1 ГБ), красный (> 1 ГБ).
- **Фильтрация kernel threads** -- kworker'ы и системные треды скрыты.
- **Ноль зависимостей** для ядра (только `console` для цветов).

## Установка

```bash
git clone https://github.com/DuvalShapli/ltop.git
cd ltop
./install.sh
```

Опционально -- системная установка:
```bash
sudo cp target/release/ltop /usr/local/bin/
```

## Использование

### Режим мониторинга (по умолчанию)

```bash
ltop
# или
ltop 10
```

Показывает топ N процессов по RSS. Обновляется по нажатию `R` (обновить) или `Q` (выйти).

### Режим убийства

```bash
ltop -k
ltop -k 10
```

Выводит статичный нумерованный список. Введи номер процесса и нажми Enter.

- `0` -- обновить список.
- `1..N` -- отправить `SIGKILL` выбранному PID.

## Архитектура

```
src/
|-- main.rs   # Аргументы, выбор режима, главные циклы
|-- proc.rs   # Парсер /proc, структура ProcessInfo, фильтр kthreads
|-- ui.rs     # Вывод в терминал, цвета, помощники ввода
```

## Лицензия

MIT. См. [LICENSE](LICENSE).
