# 🧠 Date Clipboard Converter

A lightweight Rust console application for Windows that listens for `Ctrl + Shift + Y`, reads a list of dates from the clipboard in `D/M/YYYY` format, and converts them into `M/D/YYYY` format.

When the conversion is successful, it replaces the clipboard content with the converted dates and plays a Windows notification sound.

---

## ✨ Features

- 🖱️ **Global Shortcut Listener** – Reacts to `Ctrl + Shift + Y` globally.
- 📋 **Clipboard Integration** – Reads and writes from/to the system clipboard.
- 🔁 **Date Reformatting** – Converts dates from `D/M/YYYY` to `M/D/YYYY`.
- 🔊 **Windows Sound Notification** – Plays a system "information" sound after successful conversion.
- 🐞 **Debug Mode** – Use `-d` or `--debug` to print before-and-after clipboard content.

---

## 🔧 Usage

1. Build and run the app:

   ```bash
   cargo build --release
   ./target/release/date_clip_converter -d
   ```

2. Copy a list of dates in this format to your clipboard:

   ```
   3/10/2025
   12/1/2024
   ```

3. Press **Ctrl + Shift + Y**.
4. The clipboard is updated to:

   ```
   10/3/2025
   1/12/2024
   ```

5. A **notification sound** plays to indicate success.

---

## 🖥️ Example

With `--debug` enabled:

```text
Clipboard contains:
3/10/2025
12/1/2024
Converted to:
10/3/2025
1/12/2024
```

---

## 📦 Dependencies

- [`clipboard`](https://crates.io/crates/clipboard) – Cross-platform clipboard access  
  → [GitHub Repo](https://github.com/aweinstock314/rust-clipboard)

- [`device_query`](https://crates.io/crates/device_query) – Detects global keyboard input  
  → [GitHub Repo](https://github.com/kaandedeoglu/device_query)

- [`regex`](https://crates.io/crates/regex) – Powerful regular expression engine  
  → [GitHub Repo](https://github.com/rust-lang/regex)

- [`argh`](https://crates.io/crates/argh) – Lightweight CLI argument parsing  
  → [GitHub Repo](https://github.com/google/argh)

- [`winapi`](https://crates.io/crates/winapi) – Low-level access to Windows APIs  
  → [GitHub Repo](https://github.com/retep998/winapi-rs)

---

## ⚠️ Notes

- Only runs on **Windows OS**.
- Dates must follow the `D/M/YYYY` format (e.g., `3/10/2025`).
- Multiple dates should be line-separated (`\r\n`).
- The program continues running in the console and listens for the hotkey.

---

## 📃 License

MIT License
