pub mod mandelbrot {
    use colorgrad::Gradient;
    use num_complex::Complex;
    use pix_engine::prelude::*;
    use rayon::prelude::*;

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
        colors: Gradient,
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
            colors: Gradient,
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
                colors,
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
