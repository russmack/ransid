
// ANSI escape codes. Ref: https://en.wikipedia.org/wiki/ANSI_escape_code

macro_rules! ESC {       () => ( "\x1b" ) }                 // Escape character ASCII decimal 27
macro_rules! CSI {       () => ( concat!( ESC!(), "[") ) }  // Control Sequence Introducer
macro_rules! SGRSuffix { () => ( "m" ) }                    // Select Graphic Rendition
macro_rules! SGRTmpl {   () => ( concat!(CSI!(), "%s", SGRSuffix!() ) ) }
macro_rules! TextTmpl {  () => ( concat!( SGRTmpl!(), "%s", AllOff!() ) ) }
macro_rules! AllOff {    () => ( "\x1b[0m" ) }              // Reset all attributes

// Rust doesn't do const concatenation, hence the macros above.
// const ESC: &'static str = "\x1b";                         // Escape character ASCII decimal 27
// const CSI: &'static str = concat!(env!("ESC"), "[");      // Control Sequence Introducer
// const SGRSuffix: &'static str = "m";                      // Select Graphic Rendition
// const SGRTmpl: &'static str = CSI!() + "%s" + SGRSuffix;  //
// const TextTmpl: &'static str = SGRTmpl + "%s" + AllOff;   //
// const AllOff: &'static str = "\x1b[0m";                   // Reset all attributes

// Set text decoration

const BOLD_INTENSITY: u8 = 1;     //
const UNDERLINE: u8 = 4;          //
const BLINK_SLOW: u8 = 5;         // less than 150 per minute
const IMAGE_NEGATIVE: u8 = 7;     // inverse or reverse; swap foreground and background
const NORMAL_INTENSITY: u8 = 22;  // Neither bold nor faint

// Set text color (foreground) = 30–37 ; 30 + x

const FG_BLACK: u8 = 30;
const FG_RED: u8 = 31;
const FG_GREEN: u8 = 32;
const FG_YELLOW: u8 = 33;
const FG_BLUE: u8 = 34;
const FG_MAGENTA: u8 = 35;
const FG_CYAN: u8 = 36;
const FG_GRAY: u8 = 37;

// Set background color	 = 	40–47 ; 40 + x

const BG_BLACK: u8 = 40;
const BG_RED: u8 = 41;
const BG_GREEN: u8 = 42;
const BG_YELLOW: u8 = 43;
const BG_BLUE: u8 = 44;
const BG_MAGENTA: u8 = 45;
const BG_CYAN: u8 = 46;
const BG_GRAY: u8 = 47;

pub trait Anstring {
    fn get_anstring(&self, u8) -> String;

    fn bold(&self) -> String;
    fn underline(&self) -> String;
    fn normal(&self) -> String;
    fn blink_slow(&self) -> String;
    fn negative(&self) -> String;

    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn gray(&self) -> String;

    fn bg_black(&self) -> String;
    fn bg_red(&self) -> String;
    fn bg_green(&self) -> String;
    fn bg_yellow(&self) -> String;
    fn bg_blue(&self) -> String;
    fn bg_magenta(&self) -> String;
    fn bg_cyan(&self) -> String;
    fn bg_gray(&self) -> String;
}

impl Anstring for String {
    // get_anstring constructs the string with specified ansi escape codes.
    fn get_anstring(&self, code: u8) -> String {
        let code_str: &str = &(code.to_string());
        let mut s: String = CSI!().to_owned();
        s.push_str(code_str);
        s.push_str(SGRSuffix!());
        s.push_str(&self);
        s.push_str(AllOff!());
        s
    }

    // Text decoration.

    fn bold(&self) -> String {
        self.get_anstring(BOLD_INTENSITY)
    }
    fn underline(&self) -> String {
        self.get_anstring(UNDERLINE)
    }
    fn normal(&self) -> String {
        self.get_anstring(NORMAL_INTENSITY)
    }
    fn blink_slow(&self) -> String {
        self.get_anstring(BLINK_SLOW)
    }
    fn negative(&self) -> String {
        self.get_anstring(IMAGE_NEGATIVE)
    }

    // Foreground colours.

    fn black(&self) -> String {
        self.get_anstring(FG_BLACK)
    }
    fn red(&self) -> String {
        self.get_anstring(FG_RED)
    }
    fn green(&self) -> String {
        self.get_anstring(FG_GREEN)
    }
    fn yellow(&self) -> String {
        self.get_anstring(FG_YELLOW)
    }
    fn blue(&self) -> String {
        self.get_anstring(FG_BLUE)
    }
    fn magenta(&self) -> String {
        self.get_anstring(FG_MAGENTA)
    }
    fn cyan(&self) -> String {
        self.get_anstring(FG_CYAN)
    }
    fn gray(&self) -> String {
        self.get_anstring(FG_GRAY)
    }

    // Background colours.

    fn bg_black(&self) -> String {
        self.get_anstring(BG_BLACK)
    }
    fn bg_red(&self) -> String {
        self.get_anstring(BG_RED)
    }
    fn bg_green(&self) -> String {
        self.get_anstring(BG_GREEN)
    }
    fn bg_yellow(&self) -> String {
        self.get_anstring(BG_YELLOW)
    }
    fn bg_blue(&self) -> String {
        self.get_anstring(BG_BLUE)
    }
    fn bg_magenta(&self) -> String {
        self.get_anstring(BG_MAGENTA)
    }
    fn bg_cyan(&self) -> String {
        self.get_anstring(BG_CYAN)
    }
    fn bg_gray(&self) -> String {
        self.get_anstring(BG_GRAY)
    }
}

// fn main() {
// let s: String = "hello".to_owned();
// let ss = s.bg_blue().black().blink_slow().underline();
// println!("{}", ss);
// }
//

// Omitted as poorly supported.
//
// Reset / Normal	 = 	0	  // 	all attributes off
// Faint (decreased intensity)	 = 	2	  // 	Not widely supported.
// Italic: on	 = 	3	  // 	Not widely supported. Sometimes treated as inverse.
// Blink: Rapid	 = 	6	  // 	MS-DOS ANSI.SYS; 150+ per minute; not widely supported
// Conceal	 = 	8	  // 	Not widely supported.
// Crossed-out	 = 	9	  // 	Characters legible, but marked for deletion. Not widely supported.
// Primary(default) font	 = 	10	  //
// n-th alternate font	 = 	11–19	  // 	Select the n-th alternate font (14 being the fourth alternate font, up to 19 being the 9th alternate font).
// Fraktur	 = 	20	  // 	hardly ever supported
// Bold: off or Underline: Double	 = 	21	  // 	Bold off not widely supported; double underline hardly ever supported.
// Not italic, not Fraktur	 = 	23	  //
// Underline: None	 = 	24	  // 	Not singly or doubly underlined
// Blink: off	 = 	25	  //
// Reserved	 = 	26	  //
// Image: Positive	 = 	27	  //
// Reveal	 = 	28	  // 	conceal off
// Not crossed out	 = 	29	  //
// Reserved for extended set foreground color	 = 	38	  // 	typical supported next arguments are 5;x where x is color index (0..255) or 2;r;g;b where r,g,b are red, green and blue color channels (out of 255)
// Default text color (foreground)	 = 	39	  // 	implementation defined (according to standard)
// Reserved for extended set background color	 = 	48	  // 	typical supported next arguments are 5;x where x is color index (0..255) or 2;r;g;b where r,g,b are red, green and blue color channels (out of 255)
// Default background color	 = 	49	  // 	implementation defined (according to standard)
// Reserved	 = 	50	  //
// Framed	 = 	51	  //
// Encircled	 = 	52	  //
// Overlined	 = 	53	  //
// Not framed or encircled	 = 	54	  //
// Not overlined	 = 	55	  //
// Reserved	 = 	56–59	  //
// ideogram underline or right side line	 = 	60	  // 	hardly ever supported
// ideogram double underline or double line on the right side	 = 	61	  // 	hardly ever supported
// ideogram overline or left side line	 = 	62	  // 	hardly ever supported
// ideogram double overline or double line on the left side	 = 	63	  // 	hardly ever supported
// ideogram stress marking	 = 	64	  // 	hardly ever supported
// ideogram attributes off	 = 	65	  // 	hardly ever supported, reset the effects of all of 60–64
// Set foreground text color, high intensity	 = 	90–97	  // 	aixterm (not in standard)
// Set background color, high intensity	 = 	100–107	  // 	aixterm (not in standard)
//
//
