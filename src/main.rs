use std::time::Instant;

// set run repetitions
const RUNS: usize = 65536;

fn main() {    
    let mut start;
    let first;
    let second;

    // bench setup
    let mut a: isize = 0;
    // /bench setup

    start = Instant::now();
    for _run in 0..RUNS {
        // 1st run

        a += 1;
        // a += 2;      // reverse test

        // bench function
        // func1();
        // func2();     // reverse test

    }   // /1st run
    first = start.elapsed();

    // reset bench setup
    a = 0;
    // /reset bench setup
    
    start = Instant::now();
    for _run in 0..RUNS {
        // 2nd run

        a += 2;
        // a += 1;      // reverse test

        // bench function
        // func2();
        // func1();     // reverse test

    }   // 2nd run
    second = start.elapsed();

    // print result nanoseconds
    // println!("1st: {:?} ns", first.as_secs_f32() * 1.0e9 / RUNS as f32);
    // println!("2nd: {:?} ns", second.as_secs_f32() * 1.0e9 / RUNS as f32);

    // print result microseconds
    println!("1st: {:?} µs", first.as_secs_f32() * 1.0e6 / RUNS as f32);
    println!("2nd: {:?} µs", second.as_secs_f32() * 1.0e6 / RUNS as f32);

    // print result milliseconds
    // println!("1st: {:?} ms", first.as_secs_f32() * 1000.0 / RUNS as f32);
    // println!("2nd: {:?} ms", second.as_secs_f32() * 1000.0 / RUNS as f32);

    // print result seconds
    // println!("1st: {:?} s", first.as_secs_f32() / RUNS as f32);
    // println!("2nd: {:?} s", second.as_secs_f32() / RUNS as f32);
}

fn func1() {}       // demo function

fn func2() {}       // demo function
