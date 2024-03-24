pub mod mandelbrot {

    use colorgrad::Gradient;
    use num_complex::Complex;
    use pix_engine::prelude::*;
    use rayon::prelude::*;

    static COLORS: [&str; 38] = [
        "blues",
        "br_bg",
        "bu_gn",
        "bu_pu",
        "cividis",
        "cool",
        "cubehelix",
        "gn_bu",
        "greens",
        "greys",
        "inferno",
        "magma",
        "or_rd",
        "oranges",
        "pi_yg",
        "plasma",
        "pr_gn",
        "pu_bu",
        "pu_bu_gn",
        "pu_or",
        "pu_rd",
        "purples",
        "rainbow",
        "rd_bu",
        "rd_gy",
        "rd_pu",
        "rd_yl_bu",
        "rd_yl_gn",
        "reds",
        "sinebow",
        "spectral",
        "turbo",
        "viridis",
        "warm",
        "yl_gn",
        "yl_gn_bu",
        "yl_or_br",
        "yl_or_rd",
    ];

    pub fn gradient_from_string(name: &str) -> Gradient {
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

    fn evaluate<T: Num + TryFrom<f64>>(x: T, y: T, max_iters: usize) -> f64 {
        let q = (x - from_f64(0.25)) * (x - from_f64(0.25)) + y * y;
        if q * (q + (x - from_f64(0.25))) <= y * y * from_f64(0.25) {
            return max_iters as f64;
        }
        let mut z = Complex::<T>::default();
        let mut z_old = Complex::<T>::default();
        let c = Complex::<T>::new(x, y);
        for i in 0..max_iters {
            if z.norm_sqr() >= from_f64(4.0) {
                return (i - 1) as f64;
            }
            z = z * z + c;
            if z == z_old {
                return max_iters as f64;
            }
            if i % 20 == 0 {
                z_old = z;
            }
        }
        max_iters as f64
    }

    pub struct Mandelbrot<T> {
        x_min: Vec<T>,
        x_max: Vec<T>,
        y_min: Vec<T>,
        y_max: Vec<T>,
        set_storage: Vec<Vec<f64>>,
        max_iters: usize,
        colors: String,
        show_box: bool,
        inverted: bool,
    }

    fn remap<T: Num>(val: T, a: T, b: T, c: T, d: T) -> T {
        (val - a) * (d - c) / (b - a) + c
    }

    fn from_f64<T: Num + TryFrom<f64>>(val: f64) -> T {
        match T::try_from(val) {
            Ok(v) => v,
            Err(_) => panic!("Conversion error!"),
        }
    }
    fn from_i32<T: Num + TryFrom<i32>>(val: i32) -> T {
        match T::try_from(val) {
            Ok(v) => v,
            Err(_) => panic!("Conversion error!"),
        }
    }
    fn from_u32<T: Num + TryFrom<u32>>(val: u32) -> T {
        match T::try_from(val) {
            Ok(v) => v,
            Err(_) => panic!("Conversion error!"),
        }
    }

    impl<T: Num + TryFrom<f64> + TryFrom<u32> + Sync> Mandelbrot<T> {
        pub fn new(
            height: usize,
            width: usize,
            colors: &str,
            max_iters: usize,
            inverted: bool,
        ) -> Self {
            Self {
                x_min: vec![from_f64(-2.0)],
                x_max: vec![from_f64(0.5)],
                y_min: vec![from_f64(-2.0)],
                y_max: vec![from_f64(2.0)],
                set_storage: vec![vec![0.0; height]; width],
                max_iters,
                colors: colors.to_string(),
                show_box: false,
                inverted,
            }
        }
        #[cfg(feature = "rayon")]
        pub fn update_set(&mut self, width: u32, height: u32) {
            self.set_storage = (0..width)
                .into_par_iter()
                .map(|u| {
                    let x: T = remap(
                        from_u32(u),
                        from_f64(0.0),
                        from_u32(width),
                        *self.x_min.last().unwrap(),
                        *self.x_max.last().unwrap(),
                    );
                    (0..height)
                        .map(|v| {
                            let y: T = remap(
                                from_u32(v),
                                from_f64(0.0),
                                from_u32(height),
                                *self.y_min.last().unwrap(),
                                *self.y_max.last().unwrap(),
                            );
                            evaluate(x, y, self.max_iters)
                        })
                        .collect()
                })
                .collect();
        }

        #[cfg(not(feature = "rayon"))]
        pub fn update_set(&mut self, width: u32, height: u32) {
            self.set_storage = (0..width)
                .map(|u| {
                    let x: T = remap(
                        from_u32(u),
                        from_f64(0.0),
                        from_u32(width),
                        *self.x_min.last().unwrap(),
                        *self.x_max.last().unwrap(),
                    );
                    (0..height)
                        .map(|v| {
                            let y: T = remap(
                                from_u32(v),
                                from_f64(0.0),
                                from_u32(height),
                                *self.y_min.last().unwrap(),
                                *self.y_max.last().unwrap(),
                            );
                            evaluate(x, y, self.max_iters)
                        })
                        .collect()
                })
                .collect();
        }
        fn draw_set(&self, s: &mut PixState) -> PixResult<()> {
            for u in 0..s.width()? {
                for v in 0..s.height()? {
                    let mut iters = self.set_storage[u as usize][v as usize];
                    if !self.inverted {
                        iters = self.max_iters as f64 - iters;
                    }
                    s.stroke(Color::from_slice(
                        ColorMode::Rgb,
                        gradient_from_string(&self.colors)
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

    impl<T> PixEngine for Mandelbrot<T>
    where
        T: Num + TryFrom<f64> + TryFrom<i32> + TryFrom<u32> + Sync,
    {
        fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
            s.rect_mode(RectMode::Center);
            s.fill(None);
            let width = s.width()?;
            let height = s.height()?;
            self.update_set(width, height);
            Ok(())
        }
        fn on_key_released(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
            match event.key {
                Key::Space => {
                    self.show_box = !self.show_box;
                    Ok(true)
                }
                Key::Left => {
                    let ind = COLORS
                        .iter()
                        .position(|c| c.to_string() == self.colors)
                        .unwrap();
                    let next_ind = if ind == 0 { 37 } else { ind - 1 };
                    self.colors = COLORS[next_ind].to_string();
                    s.redraw();
                    Ok(true)
                }
                Key::Right => {
                    let ind = COLORS
                        .iter()
                        .position(|c| c.to_string() == self.colors)
                        .unwrap();
                    let next_ind = if ind == 37 { 0 } else { ind + 1 };
                    self.colors = COLORS[next_ind].to_string();
                    s.redraw();
                    Ok(true)
                }
                Key::Tab => {
                    self.inverted = !self.inverted;
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
            let width = s.width()?;
            let height = s.height()?;
            if let Mouse::Left = btn {
                let x0 = remap(
                    from_i32(pos.x() - 50),
                    from_f64(0.0),
                    from_u32(s.width()?),
                    *self.x_min.last().unwrap(),
                    *self.x_max.last().unwrap(),
                );
                let y0 = remap(
                    from_i32(pos.y() - 50),
                    from_f64(0.0),
                    from_u32(s.width()?),
                    *self.y_min.last().unwrap(),
                    *self.y_max.last().unwrap(),
                );
                let x1 = remap(
                    from_i32(pos.x() + 50),
                    from_f64(0.0),
                    from_u32(s.width()?),
                    *self.x_min.last().unwrap(),
                    *self.x_max.last().unwrap(),
                );
                let y1 = remap(
                    from_i32(pos.y() + 50),
                    from_f64(0.0),
                    from_u32(s.width()?),
                    *self.y_min.last().unwrap(),
                    *self.y_max.last().unwrap(),
                );
                self.x_min.push(x0);
                self.x_max.push(x1);
                self.y_min.push(y0);
                self.y_max.push(y1);
                self.update_set(width, height);
                s.redraw();
            }

            if let Mouse::Right = btn {
                if self.x_min.len() > 1 {
                    self.x_min.pop();
                    self.x_max.pop();
                    self.y_min.pop();
                    self.y_max.pop();
                    self.update_set(width, height);
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
}
