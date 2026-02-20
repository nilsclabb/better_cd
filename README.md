# better_cd

### A better `cd` for lazy people like me

I never liked the `cd` command. I never remember how to use it properly. So I made this — an interactive, arrow-key-driven directory navigator for the terminal, built with [Ratatui](https://ratatui.rs/). Type `cd+`, browse around, hit Enter, and you're there.

> **Note:** this thing was vibed in in about 2.5 minutes. Use at your own risk!

---

## Prerequisites

- **Rust & Cargo** — Install via [rustup](https://rustup.rs/) if you don't have them:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **A zsh (or bash) shell** — the `cd+` wrapper function is a small shell function you add to your config.

---

## Installation

### 1. Clone the repo

```sh
git clone https://github.com/nilsclabb/better_cd.git
cd better_cd
```

### 2. Build the release binary

```sh
cargo build --release
```

This produces the binary at `./target/release/better_cd`.

### 3. Add the `cd+` shell function

Because a subprocess can't change your shell's working directory, `better_cd` prints the selected path to stdout and a tiny shell wrapper does the actual `cd`. Add the following to your **`~/.zshrc`** (or `~/.bashrc`):

```sh
function cd+() {
    local DEST=$("$HOME/path/to/better_cd/target/release/better_cd" "$@")
    if [ -n "$DEST" ]; then
        cd "$DEST"
    fi
}
```

> **Important:** Replace `$HOME/path/to/better_cd` with the actual absolute path to where you cloned the repo—for example `$HOME/tools/better_cd`.

### 4. Reload your shell

```sh
source ~/.zshrc
```

---

## Usage

```sh
cd+
```

An interactive TUI opens in your terminal. Navigate to the directory you want, press **Enter**, and your shell will `cd` into it.

---

## Keybindings

| Key | Action |
|---|---|
| `↑` / `↓` | Move selection up / down |
| `→` | Enter the selected directory |
| `←` | Go to parent directory |
| `Enter` | Confirm selection and `cd` into it |
| `Esc` | Quit without changing directory |
| Any letter | Live-filter the directory list |
| `Backspace` | Delete last character from filter |

---

## License

MIT License
