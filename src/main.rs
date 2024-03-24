use clap::Parser;
use fpdec::Decimal;
use mandelbrot::mandelbrot::Mandelbrot;
use pix_engine::prelude::*;

#[derive(Parser, Debug)]
///
/// Color options:
/// cividis, cool, cubehelix, inferno, magma, plasma,
/// rainbow, sinebow, spectral, turbo, viridis, warm,
/// blues, greens, oranges, reds, purples, greys,
/// br_bg, bu_gn, bu_pu, gn_bu, or_rd,
/// pi_yg, pr_gn, pu_bu, pu_bu_gn, pu_or,
/// pu_rd, rd_bu, rd_gy, rd_pu, rd_yl_bu,
/// rd_yl_gn, yl_gn, yl_gn_bu, yl_or_br, yl_or_rd
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "magma")]
    color: String,

    #[arg(short, long, default_value_t = 600)]
    height: usize,

    #[arg(short, long, default_value_t = 600)]
    width: usize,

    #[arg(short, long, default_value_t = 500)]
    max_iters: usize,

    #[arg(long)]
    inverse: bool,

    #[arg(long)]
    // Slower, but uses fixed decimal numbers for higher zoom accuracy
    precise: bool,
}

fn main() -> PixResult<()> {
    let args = Args::parse();
    let mut engine = Engine::builder()
        .dimensions(args.width as u32, args.height as u32)
        .title("Mandelbrot")
        .build()?;
    if args.precise {
        let mut app = Mandelbrot::<Decimal>::new(
            args.width,
            args.height,
            &args.color,
            args.max_iters,
            args.inverse,
        );
        engine.run(&mut app)
    } else {
        let mut app = Mandelbrot::<f64>::new(
            args.width,
            args.height,
            &args.color,
            args.max_iters,
            args.inverse,
        );
        engine.run(&mut app)
    }
}
