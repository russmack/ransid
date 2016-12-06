
extern crate ransid;

use ransid::Anstring;

fn main() {
    let s: String = "hello".to_owned();
    let ss = s.bg_blue().blink_slow().underline();
    println!("{}", ss);
    let sss = ransid::DRAW_SKULL_CROSSBONES;
    println!("{}", sss);
    let ssss = ransid::DRAW_HORIZONTAL_LONG;
    println!("{}", ssss);
}
