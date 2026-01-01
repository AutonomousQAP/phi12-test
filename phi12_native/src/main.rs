use std::hint::black_box;
use std::time::Instant;

fn phi12_fractal(iter: u32) -> (f64, f64) {
    let mut sum = 0.0f64;
    let start = Instant::now();

    for i in 0..iter {
        let re = (i as f64 / 1000.0).sin();
        let im = (1.618033988749895 * re).cos();
        sum += (re * im + 321.997 / 1000.0).abs();
    }

    let elapsed_secs = start.elapsed().as_secs_f64();
    black_box(sum); // Force the compiler to compute sum
    (sum, iter as f64 / elapsed_secs)
}

fn main() {
    let iterations: u32 = 100_000_000; // 100M for ~seconds runtime, accurate avg
    let (sum, ops_sec) = phi12_fractal(iterations);

    println!("φ¹² Native Rust (Termux): {:.0} ops/sec", ops_sec);
    println!("Iterations: {} | Sum: {:.3}", iterations, sum);
}
