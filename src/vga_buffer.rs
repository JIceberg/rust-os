#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

// this takes the foreground as an unsigned byte, left shifted by 4, and then the OR
// operator is used with the background as an unsigned byte.
// ---------------------
// if our foreground is blue, and our background is light red,
// then this would be (0b00000001 << 4) | 0b00001100 which is
// 0b00010000 | 0b00001100 which finalizes as 0b00011100.
// ---------------------
// This creates a byte where the first four bits represent the foreground
// color and the last four bits represent the background color. This makes
// sending the color code to the framework much easier.
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// here is the public struct for writing the byte to the framework
// ---------------------
// The writer will always write to the last line until a newline is called
// or the line is full. Then it will shift up. The lifetime is specified as 'static
// to show that the reference is valid for the entire run time.
pub struct Writer {
    column_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    // this is the function for writing a byte (char) to the buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),   // if the newline character is called, then we create a new line
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();    // if the line is filled, then we create a new line
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                // this creates the character and adds it to the screen which
                // is represented through the VGA buffer as a matrix.
                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code,
                };
                self.column_pos += 1;
            }
        }
    }

    // this is the function for writing entire strings to the buffer
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),   // write each individual byte
                // not part of the printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        /* TODO */
    }

}

pub fn print_something(s: &str) {
    let mut writer = Writer {
        column_pos: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string(s);
}