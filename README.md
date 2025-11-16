# port_scanner_x — A Simple Rust-based Port Scanner ✨

## 🔥 Overview
**port_scanner_x** is a simple and fast port scanner application built with Rust. Its goal is speed, simplicity, making it perfect for developers or anyone looking for a small but powerful tool.

---

## 🚀 Features
- Extremely fast and lightweight (Rust 🦀).
- Clean and simple command-line interface.
- Extensible and fully tested with `cargo test`.

---

## 🧩 Installation (for developers)
To install from source:

```bash
# Clone the repository
git clone https://github.com/Hoseinpy/port_scanner_x.git
cd port_scanner_x

# Build and run (development mode)
cargo run -- [args]

# Or build a release version and use the binary
cargo build --release
# Binary will be in target/release/port_scanner_x
```

To install via `cargo install` (when published):

```bash
cargo install --git https://github.com/Hoseinpy/port_scanner_x.git --rev main
# Or from crates.io when available:
# cargo install port_scanner_x
```

---

## ⚙️ Usage (CLI)
Quick examples:

```bash
# scan all range (0..65535) for given address
port_scanner_x -a 127.0.0.1 # note: if run without -a. is running default on localhost

# scan all range (0..65535) for given host
port_scanner_x -h example.com

# scan for custom range
port_scanner_x -s 1 -e 100 # -s is start port and -e is end port
```

---

## 🧪 Testing
All tests can be run with `cargo test`:

```bash
# Run the test suite
cargo test
```

Please make sure all tests pass before submitting a Pull Request ✅

---

## 🛠️ Contributing
Want to help? Awesome! Follow these steps for a smooth contribution process:

1. Fork and work on a separate branch (`feature/<something>`).
2. Run `cargo fmt` and `cargo clippy` before your PR.
3. Ensure all tests pass.
4. Submit a clear PR explaining what issue it fixes or what feature it adds.

Keep issues/PRs simple and clear: what happened, steps to reproduce, and suggested solution.

---

## 📜 License
This project is licensed under **MIT** — free to use and modify.

---

## ❤️ Thanks
If port_scanner_x helped you or you like it, give it a ⭐. Report bugs or suggest features via issues
