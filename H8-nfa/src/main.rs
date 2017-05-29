//! A naive implementation of an NFA in Rust.

mod nfa;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

use nfa::*;

fn main() {
    let mut nfa = NFA::new(0..35);
    let file = File::open("./H8.trans")
        .expect("Transition file 'H8.trans' not found!");

    let tokens = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|line| {
            let mut parts = line.split_whitespace();
            let from: u8 = parts.next().unwrap().parse().unwrap();
            let with = parts.next().unwrap().chars().next().unwrap();
            let to: u8 = parts.next().unwrap().parse().unwrap();

            (from, with, to)
        });

    for (from, with, to) in tokens {
        nfa.add_transition(from, with, to);
    }

    println!("Please enter the simulation start:");
    let mut start = String::new();
    io::stdin().read_line(&mut start).unwrap();
    let start = start.trim().parse().unwrap();

    println!("Please enter your word:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let chars = input.trim().chars().collect::<Vec<_>>();

    println!("Simulating...");
    let result = nfa.simulate(start, &chars);
    println!("Simulated {}, got into {:?}.", input.trim(), result);
}
