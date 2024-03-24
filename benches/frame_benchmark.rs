use criterion::{criterion_group, criterion_main, Criterion};
use mandelbrot::mandelbrot::Mandelbrot;

fn frame_benchmark(c: &mut Criterion) {
    let mut m: Mandelbrot<f64> = Mandelbrot::new(600, 600, colorgrad::plasma(), 10000, true);
    c.bench_function("Calculate", |b| b.iter(|| m.update_set(600, 600)));
}

criterion_group!(benches, frame_benchmark,);
criterion_main!(benches);
