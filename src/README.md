# ransid

Rust library providing colour and style for the terminal.

## Usage
Run the example:
```
cargo run examples/example.go
```
Use the library:
```
let s = "I am black on green.".black().bg_green().underline();
println!("{}", s);

Or

let style = new_style().gray().bg_blue().bold().blink_slow();
println!(style.render("I am white on blue."));
```

## License
BSD 3-Clause: [LICENSE.txt](LICENSE.txt)

[<img alt="LICENSE" src="http://img.shields.io/pypi/l/Django.svg?style=flat-square"/>](LICENSE.txt)
