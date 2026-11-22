# Build & Development Instructions — Tiny68

## 1. Prerequisites
- Windows 10 or Windows 11 x64
- Stable Rust toolchain (Rust 1.80+ or 1.99+)
- Git
- Optional: Inno Setup 6 (for building the `.exe` installer)

---

## 2. Building Tiny68

### Clone repository:
```cmd
git clone https://github.com/tiny68/tiny68.git
cd tiny68
```

### Build debug binaries:
```cmd
cargo build --workspace
```

### Build optimized release binaries:
```cmd
cargo build --workspace --release
```
The compiled binaries are generated in `target\release\`:
- `target\release\tiny68.exe` (~1.3 MB)
- `target\release\tiny68-settings.exe` (~314 KB)

---

## 3. Running Automated Checks

### Run Linter (Clippy):
```cmd
cargo clippy --workspace --all-targets -- -D warnings
```

### Run Formatter:
```cmd
cargo fmt --all -- --check
```

### Run Test Suite:
```cmd
cargo test --workspace
```

### Run Performance Benchmarks:
```cmd
cargo run --bin benchmark_runner
```

---

## 4. Building the Installer
Using Inno Setup:
```cmd
iscc installer\tiny68_setup.iss
```
The setup installer executable will be written to `dist\Tiny68_Setup_v0.1.0.exe`.

