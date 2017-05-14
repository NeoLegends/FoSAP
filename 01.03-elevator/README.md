# Elevator State Machine

This is a simple program that reads lines containing the letters
U, D, A, Z and R from its standard input and simulates an elevator
based on that. U means up, D down, A open door, Z close door and R
call emergency.

Please download the test files from
1. http://tcs.rwth-aachen.de/lehre/FSAP/SS2017/H3-easy.txt
2. or http://tcs.rwth-aachen.de/lehre/FSAP/SS2017/H3-hard.gz

The second link points to a gzipped example that's meant for
benchmarking puposes. This algorithm managed to parse the entire
gunzipped file in ~3m30s on an i5 Surface Pro 4.

## Usage

Execute the following from bash / git bash on Windows:
```bash
cargo build --release --all
time cat H3-easy.txt | ./target/release/elevator.exe # or
time gzip -cd H3-hard.gz | ./target/release/elevator.exe
```
