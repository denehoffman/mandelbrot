use clap::Parser;
use colorgrad::Gradient;
use num_complex::Complex;
use pix_engine::prelude::*;
use rayon::prelude::*;
use rust_decimal::prelude::*;

type Dec = f64;
type ComplexDec = Complex<Dec>;

fn evaluate(x: Dec, y: Dec, max_iters: usize) -> f64 {
    let mut z = ComplexDec::default();
    let c = ComplexDec::new(x, y);
    for i in 0..max_iters {
        if z.norm_sqr() >= Dec::from_f64(4.0).unwrap() {
            return (i - 1) as f64;
        }
        z = z * z + c;
    }
    max_iters as f64
}

struct Mandelbrot {
    x_min: Vec<Dec>,
    x_max: Vec<Dec>,
    y_min: Vec<Dec>,
    y_max: Vec<Dec>,
    set_storage: Vec<Vec<f64>>,
    max_iters: usize,
    colors: Gradient,
    show_box: bool,
}

fn remap<T: Num>(val: T, a: T, b: T, c: T, d: T) -> T {
    (val - a) * (d - c) / (b - a) + c
}

impl Mandelbrot {
    fn new(height: usize, width: usize, colors: Gradient) -> Self {
        Self {
            x_min: vec![Dec::from_f64(-2.0).unwrap()],
            x_max: vec![Dec::from_f64(0.5).unwrap()],
            y_min: vec![Dec::from_f64(-2.0).unwrap()],
            y_max: vec![Dec::from_f64(2.0).unwrap()],
            set_storage: vec![vec![0.0; height]; width],
            max_iters: 2000,
            colors,
            show_box: false,
        }
    }
    fn update_set(&mut self, s: &mut PixState) -> PixResult<()> {
        let height = s.height()?;
        let width = s.width()?;
        self.set_storage = (0..width)
            .into_par_iter()
            .map(|u| {
                let x = remap(
                    Dec::from(u),
                    Dec::from_f64(0.0).unwrap(),
                    Dec::from(width),
                    *self.x_min.last().unwrap(),
                    *self.x_max.last().unwrap(),
                );
                (0..height)
                    .map(|v| {
                        let y = remap(
                            Dec::from(v),
                            Dec::from_f64(0.0).unwrap(),
                            Dec::from(height),
                            *self.y_min.last().unwrap(),
                            *self.y_max.last().unwrap(),
                        );
                        evaluate(x, y, self.max_iters)
                    })
                    .collect()
            })
            .collect();
        Ok(())
    }
    fn draw_set(&self, s: &mut PixState) -> PixResult<()> {
        for u in 0..s.width()? {
            for v in 0..s.height()? {
                let iters = self.set_storage[u as usize][v as usize];
                s.stroke(Color::from_slice(
                    ColorMode::Rgb,
                    self.colors
                        .at(iters / self.max_iters as f64)
                        .to_array()
                        .into_iter()
                        .map(|v| v * 255.0)
                        .collect::<Vec<_>>(),
                )?);
                s.point(point!(u as i32, v as i32))?;
            }
        }
        Ok(())
    }
}

impl PixEngine for Mandelbrot {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.rect_mode(RectMode::Center);
        s.fill(None);
        self.update_set(s)?;
        Ok(())
    }
    fn on_key_released(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match event.key {
            Key::Space => {
                self.show_box = !self.show_box;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
    fn on_mouse_clicked(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: Point<i32>,
    ) -> PixResult<bool> {
        if let Mouse::Left = btn {
            let x0 = remap(
                Dec::from(pos.x() - 50),
                Dec::from_f64(0.0).unwrap(),
                Dec::from((s.width())?),
                *self.x_min.last().unwrap(),
                *self.x_max.last().unwrap(),
            );
            let y0 = remap(
                Dec::from(pos.y() - 50),
                Dec::from_f64(0.0).unwrap(),
                Dec::from((s.width())?),
                *self.y_min.last().unwrap(),
                *self.y_max.last().unwrap(),
            );
            let x1 = remap(
                Dec::from(pos.x() + 50),
                Dec::from_f64(0.0).unwrap(),
                Dec::from((s.width())?),
                *self.x_min.last().unwrap(),
                *self.x_max.last().unwrap(),
            );
            let y1 = remap(
                Dec::from(pos.y() + 50),
                Dec::from_f64(0.0).unwrap(),
                Dec::from((s.width())?),
                *self.y_min.last().unwrap(),
                *self.y_max.last().unwrap(),
            );
            self.x_min.push(x0);
            self.x_max.push(x1);
            self.y_min.push(y0);
            self.y_max.push(y1);
            self.update_set(s)?;
            s.redraw();
        }

        if let Mouse::Right = btn {
            if self.x_min.len() > 1 {
                self.x_min.pop();
                self.x_max.pop();
                self.y_min.pop();
                self.y_max.pop();
                self.update_set(s)?;
                s.redraw();
            }
        }
        Ok(true)
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.draw_set(s)?;
        if self.show_box {
            s.stroke(Color::RED);
            s.rect(rect![s.mouse_pos(), 100, 100])?;
        }
        Ok(())
    }
}

fn gradient_from_string(name: &str) -> Gradient {
    match name {
        "blues" => colorgrad::blues(),
        "br_bg" => colorgrad::br_bg(),
        "bu_gn" => colorgrad::bu_gn(),
        "bu_pu" => colorgrad::bu_pu(),
        "cividis" => colorgrad::cividis(),
        "cool" => colorgrad::cool(),
        "cubehelix" => colorgrad::cubehelix_default(),
        "gn_bu" => colorgrad::gn_bu(),
        "greens" => colorgrad::greens(),
        "greys" => colorgrad::greys(),
        "inferno" => colorgrad::inferno(),
        "magma" => colorgrad::magma(),
        "or_rd" => colorgrad::or_rd(),
        "oranges" => colorgrad::oranges(),
        "pi_yg" => colorgrad::pi_yg(),
        "plasma" => colorgrad::plasma(),
        "pr_gn" => colorgrad::pr_gn(),
        "pu_bu" => colorgrad::pu_bu(),
        "pu_bu_gn" => colorgrad::pu_bu_gn(),
        "pu_or" => colorgrad::pu_or(),
        "pu_rd" => colorgrad::pu_rd(),
        "purples" => colorgrad::purples(),
        "rainbow" => colorgrad::rainbow(),
        "rd_bu" => colorgrad::rd_bu(),
        "rd_gy" => colorgrad::rd_gy(),
        "rd_pu" => colorgrad::rd_pu(),
        "rd_yl_bu" => colorgrad::rd_yl_bu(),
        "rd_yl_gn" => colorgrad::rd_yl_gn(),
        "reds" => colorgrad::reds(),
        "sinebow" => colorgrad::sinebow(),
        "spectral" => colorgrad::spectral(),
        "turbo" => colorgrad::turbo(),
        "viridis" => colorgrad::viridis(),
        "warm" => colorgrad::warm(),
        "yl_gn" => colorgrad::yl_gn(),
        "yl_gn_bu" => colorgrad::yl_gn_bu(),
        "yl_or_br" => colorgrad::yl_or_br(),
        "yl_or_rd" => colorgrad::yl_or_rd(),
        _ => panic!("Unsupported colorscheme name!"),
    }
}

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
}

fn main() -> PixResult<()> {
    let args = Args::parse();
    let mut engine = Engine::builder()
        .dimensions(args.width as u32, args.height as u32)
        .title("Mandelbrot")
        .build()?;
    let mut app = Mandelbrot::new(args.width, args.height, gradient_from_string(&args.color));
    engine.run(&mut app)
}
