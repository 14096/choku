# CHOKU

Terminal colors and styling for rust.  
This library is based on [chalk](https://github.com/michaeldoaty/chalk).

### USAGE

```rust
fn main() {
  // use default colors
  let red_yellow = Choku::new(Color::Default(Red), "Hello, World!")
    .underline()
    .bg(Cyan)
    .bold()
    .color();

  // use custom RGB color
  let custom_rgb = Choku::new(Color::RGB(0, 230, 0), "Custom RGB Color")
    .color();

  println!("{}", red_yellow);
  println!("{}", custom_rgb);
}
```
