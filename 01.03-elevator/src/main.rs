//! A fictional elevator state machine.

use std::io::{self, BufReader, Read};

const CLOSE: u8 = 'Z' as u8;
const DOWN: u8 = 'D' as u8;
const EMERG: u8 = 'R' as u8;
const OPEN: u8 = 'A' as u8;
const UP: u8 = 'U' as u8;

const CR: u8 = '\r' as u8;
const LF: u8 = '\n' as u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum States {
    Start,
    Closed0, Open0,
    Closed1, Open1,
    Closed2, Open2,
    Closed3, Open3,
    Closed4, Open4,
}

impl States {
    pub fn is_end(&self) -> bool {
        use States::*;

        match *self {
            Start | Open0 | Open1 | Open2 | Open3 | Open4 => true,
            _ => false
        }
    }
}

fn main() {
    use States::*;

    let mut state = Start;
    let mut data = BufReader::new(io::stdin())
        .bytes()
        .map(|maybe_byte| maybe_byte.expect("Failed to read input!"))
        .peekable();

    while let Some(word) = data.next() {
        match word {
            CLOSE => {
                match state {
                    Open0 => state = Closed0,
                    Open1 => state = Closed1,
                    Open2 => state = Closed2,
                    Open3 => state = Closed3,
                    Open4 => state = Closed4,
                    _ => {}
                }
            },

            DOWN => {
                match state {
                    Closed1 => state = Closed0,
                    Closed2 => state = Closed1,
                    Closed3 => state = Closed2,
                    Closed4 => state = Closed3,
                    _ => {}
                }
            },

            OPEN if state == Start => state = Open0,

            EMERG | OPEN => {
                match state {
                    Closed0 => state = Open0,
                    Closed1 => state = Open1,
                    Closed2 => state = Open2,
                    Closed3 => state = Open3,
                    Closed4 => state = Open4,
                    _ => {}
                }
            },

            UP => {
                match state {
                    Closed0 => state = Closed1,
                    Closed1 => state = Closed2,
                    Closed2 => state = Closed3,
                    Closed3 => state = Closed4,
                    _ => {}
                }
            },

            CR | LF => {
                // Skip EOL characters
                while let Some(w) = data.peek().map(|r| *r) {
                    if w == CR || w == LF {
                        data.next();
                    } else {
                        break;
                    }
                }

                // Only output here if we're not at the end of the file.
                // For EOF we have a special case down at the bottom.
                if data.peek().is_some() {
                    println!("Reached end of line with end state: {}.", state.is_end());
                    state = Start;
                }
            },

            0...19 => {},

            _ => {
                println!("Got unknown input '{}'.", word);
            }
        }
    }

    println!("Reached end of file with end state: {}.", state.is_end());
}
