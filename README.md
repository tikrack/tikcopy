<h1 align="center">
  <img width="150" alt="TikCopy" src="https://github.com/user-attachments/assets/c8159c90-be1c-402d-b79e-125a3a99d074" />
  <br />
  TikCopy
</h1>


<p align="center">
  <b>A simple yet powerful clipboard history manager for Linux (CLI version)</b><br/>
  Store, view, and reuse your clipboard entries — like <code>Windows + V</code> but in your terminal.
</p>

---

## 🚀 Features

- 🔁 Keep history of your clipboard (up to 50 items)
- 🧠 Reuse any past entry with a simple command
- 📋 Add directly from clipboard or via <code>stdin</code>
- 🧼 Clean and readable CLI with colored output
- 🐧 Linux first — fast, native, and Rust-powered!

---

## 📦 Installation

### ✅ Via Cargo (Recommended)

```bash
cargo install --git https://github.com/YOUR_USERNAME/tikcopy
```
Or if you have the code locally:
```bash
git clone https://github.com/YOUR_USERNAME/tikcopy
cd tikcopy
cargo install --path .
```

---

## 📁 Manual (Release Binary)

```bash
cargo build --release
sudo cp target/release/tikcopy /usr/local/bin/
```

---

## ⚙ Usage

```bash
tikcopy add                 # Add current clipboard content to history
echo "Hello" | tikcopy add  # Add piped content to history
tikcopy list                # List all stored clipboard entries
tikcopy use 2               # Use 2nd item in clipboard
tikcopy delete 1            # Delete 1st item from history
```

📂 History is stored in:
`~/.local/share/tikcopy_history.json`

---

## 📸 Screenshot

```bash
$ tikcopy list
1: Hello world!
2: copied text
3: some other clipboard item
```

---

## 🤝 Contributions

Feel free to open issues, PRs, or share ideas.
If you like the project, consider starring ⭐️ it to support!

---

## 🪪 License

MIT License © 2025 [Tikrack](https://github.com/tikrack)
