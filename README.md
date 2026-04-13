# ASCII Art Renderer 🎨

A terminal-based image-to-ASCII converter written in Rust. Renders images as colored ASCII art directly in your terminal, with streaming output and full RGB color support.

## Example Output

```
@@@@$$$$BBBB####****oooo....    ....oooo****####BBBB$$$$@@@@
```
*(Colors are rendered live in the terminal)*

## Features

- Converts any image (JPG, PNG, etc.) to ASCII art
- Full RGB color per character using terminal ANSI escape codes
- Streaming output — characters appear one by one
- Adjustable terminal width and height constants

## Requirements

- Rust (stable)
- A terminal with ANSI color support (Windows Terminal, iTerm2, most Linux terminals)

## Dependencies

```toml
[dependencies]
image = "..."
crossterm = "..."
```

## Setup

1. Clone the repository
2. Add your image to the project root
3. Create an `ascii_map.txt` file in the project root with your desired ASCII gradient, from dark to light, for example:
```
 .'`^",;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$
```
4. Update the filename in `main.rs` to match your image:
```rust
let mut _img = image::open("your_image.jpg").unwrap();
```
5. Run:
```bash
cargo run --release
```

## Configuration

At the top of `main.rs` you can adjust the output resolution:

```rust
const TERMINAL_WIDTH: u32 = 220;
const TERMINAL_HEIGHT: u32 = 40;
```

Set these to match your terminal size for best results. Since terminal characters are roughly twice as tall as they are wide, the image will naturally appear proportional.

## How It Works

1. The image is loaded and resized to fit the terminal dimensions
2. Each pixel's brightness is calculated from its RGB values
3. The brightness maps to a character in the ASCII map
4. The character is printed twice side-by-side (to compensate for character aspect ratio)
5. Each character is colored with the original pixel's RGB color using crossterm

## Tips

- Use `--release` for faster rendering
- Larger images with high `TERMINAL_WIDTH` values give more detail
- Try different ASCII maps for different visual styles
- If colors look inverted, reverse the order of characters in your `ascii_map.txt`
