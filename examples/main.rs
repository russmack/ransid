
extern crate ransid;

use ransid::Anstring;

fn main() {
    let s: String = "hello".to_owned();
    let ss = s.bg_blue().blink_slow().underline();
    println!("{}", ss);
}
