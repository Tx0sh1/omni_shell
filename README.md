# Omni-Shell: A Minimal Shell in Rust

Omni-Shell is a tiny, functional shell built from scratch using Rust. It serves as a basic command-line interface that reads, evaluates, and executes user input. The goal of this project is to understand how a shell works and to explore core Rust concepts such as process management, error handling, and system-level programming.

---

## Features
- Customizable prompt (`omni-shell>`)
- Reads and executes user commands
- Built-in commands:
  - `exit` — Exit the shell
  - `cd [path]` — Change directory
  - `pwd` — Print current working directory
- Executes external commands like `ls`, `echo`, etc.
- Supports basic error handling (invalid commands, etc.)

---

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/omni-shell.git
    ```

2. Navigate to the project folder:
    ```bash
    cd omni-shell
    ```

3. Build the project using Cargo:
    ```bash
    cargo build
    ```

4. Run the shell:
    ```bash
    cargo run
    ```

---

 ## Planned Features
1. Piping (command1 | command2)

2. Background execution (command &)

3. Command history

4. File redirection (>, <)
