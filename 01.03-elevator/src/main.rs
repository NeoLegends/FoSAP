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
    Closed0, Open0,
    Closed1, Open1,
    Closed2, Open2,
    Closed3, Open3,
    Closed4, Open4,
}

fn main() {
    use States::*;

    let mut state = Closed0;
    let mut data = BufReader::new(io::stdin())
        .bytes()
        .map(|maybe_byte| maybe_byte.expect("Failed to read input!"));

    while let Some(word) = data.next() {
        if word == LF || word == CR {
            println!("Reached end of line. Ended up in {:?}.", state);
            state = Closed0;
        }

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

            _ => {
                println!("Got unknown input '{}'.", word);
            }
        }
    }
}
