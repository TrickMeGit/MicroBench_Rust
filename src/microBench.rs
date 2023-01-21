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
    // println!("1st run: {:?} ns", first.as_secs_f32() / RUNS as f32 * 1.0e9);
    // println!("2nd run: {:?} ns", second.as_secs_f32() / RUNS as f32 * 1.0e9);

    // print result microseconds
    println!("1st run: {:?} µs", first.as_secs_f32() / RUNS as f32 * 1.0e6);
    println!("2nd run: {:?} µs", second.as_secs_f32() / RUNS as f32 * 1.0e6);

    // print result milliseconds
    // println!("1st run: {:?} ms", first.as_secs_f32() / RUNS as f32 * 1000.0);
    // println!("2nd run: {:?} ms", second.as_secs_f32() / RUNS as f32 * 1000.0);

    // print result seconds
    // println!("1st run: {:?} s", first.as_secs_f32() / RUNS as f32);
    // println!("2nd run: {:?} s", second.as_secs_f32() / RUNS as f32);
}

fn func1() {}       // demo function

fn func2() {}       // demo function
