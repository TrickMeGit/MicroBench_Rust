use std::time::Instant;

// add uses for bench:


fn main() {
    // set run repetitions:
    const RUNS: u64 = 65536;

    let mut start = Instant::now();
    let first;
    let second;

    // add local variables for bench:

    let mut _a: i64 = 0;

    
    start = Instant::now();
    for _run in 0..RUNS {
        // 1st run

        _a += 1;
        // _a += 2;     // reverse test

        // bench functions
        func1();
        // func2();     // reverse test
    }
    first = start.elapsed();

    // reset bench variables:

    _a = 0;
    

    start = Instant::now();
    for _run in 0..RUNS {
        // 2nd run

        _a += 2;
        // _a += 1;     // reverse test

        // bench functions
        func2();
        // func1();     // reverse test
    }
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


// add functions for bench:

fn func1() {}       // demo function

fn func2() {}       // demo function
