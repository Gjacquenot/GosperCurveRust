use plotters::prelude::*;

type Error = Box<dyn std::error::Error + 'static>;

enum SegmentType {
    Type1,
    Type2,
}

struct Level {
    id: u8,
    scale: f64,
    directions: Vec<u8>,
    types: Vec<SegmentType>,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            id: 0_u8,
            scale: 7.0_f64.sqrt() / 2.0,
            directions: vec![0],
            types: vec![SegmentType::Type1],
        }
    }
}

fn create_the_vector_of_levels() -> Vec<Level> {
    vec![Level::default()]
}

impl Level {
    /// Create a new level from an existing one.
    ///
    /// This new level will have seven times more segment than its parent
    /// level.
    fn next(&self) -> Self {
        // Each segment will be replaced by seven new ones
        // with directions depending on the type of their
        // parent.

        // New directions for a segment of type 1
        // t1 = 'abbaaab'
        static D1: [u8; 7] = [0, 5, 3, 4, 0, 0, 1];

        // New directions for a segment of type 2
        // t2 = 'abbbaab'
        static D2: [u8; 7] = [1, 0, 0, 4, 3, 5, 0];

        let mut types = Vec::new();
        let mut directions = Vec::new();
        for (ty, dir) in self.types.iter().zip(&self.directions) {
            for j in 0..7 {
                match ty {
                    SegmentType::Type1 => {
                        types.push(match j {
                            0 | 3 | 4 | 5 => SegmentType::Type1,
                            1 | 2 | 6 => SegmentType::Type2,
                            _ => unreachable!(),
                        });
                        directions.push((dir + D1[j]) % 6);
                    }
                    SegmentType::Type2 => {
                        types.push(match j {
                            0 | 4 | 5 => SegmentType::Type1,
                            1 | 2 | 3 | 6 => SegmentType::Type2,
                            _ => unreachable!(),
                        });
                        directions.push((dir + D2[j]) % 6);
                    }
                }
            }
        }
        Level {
            id: self.id + 1,
            scale: self.scale / 7.0_f64.sqrt(),
            directions,
            types,
        }
    }

    /// Convert the formal description of a level to a x, y curve.
    ///
    /// Result is casted to a vector of tuples of float32 so that
    /// it can directly be used by the plotting library
    fn generate(&self) -> Vec<(f32, f32)> {
        let scale = self.scale;
        let n = self.directions.len();
        let mut x = vec![0.0_f64; n + 1];
        let mut y = vec![0.0_f64; n + 1];
        for i in 0..n {
            x[i + 1] = x[i] + scale * cosinus(self.directions[i]);
            y[i + 1] = y[i] + scale * sinus(self.directions[i]);
        }

        let index = self.id as f64;
        let rotation_angle = index * ((3.0_f64.sqrt() / 5.0).atan());
        rotate_and_cast(&x, &y, rotation_angle)
    }

    fn plot(&self) -> Result<(), Error> {
        let filename = format!("{}.png", self.id);
        let root = BitMapBackend::new(&filename, (1280, 1280)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(1)
            .build_ranged(-0.25..(7.0f32.sqrt() / 2.0 + 0.25), -1.25f32..0.75)?;

        chart.draw_series(LineSeries::new(self.generate(), &BLACK))?;

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .draw()?;
        Ok(())
    }
}

fn append_new_level(levels: &mut Vec<Level>) {
    if let Some(last_level) = levels.last() {
        let new_level = last_level.next();
        levels.push(new_level);
    }
}

fn create_gosper_fractal(max_level: u8) -> Vec<Level> {
    let mut levels = create_the_vector_of_levels();
    for _ in 0..max_level {
        append_new_level(&mut levels);
    }
    levels
}

/// Returns the cosinus value of an angle described by a key that is a
/// multiple of pi/3.
///
/// Here, k1 = cos(pi/3)
fn cosinus(key: u8) -> f64 {
    let k1 = 0.5;
    match key {
        0 => 1.0,
        1 => k1,
        2 => -k1,
        3 => -1.0,
        4 => -k1,
        5 => k1,
        _ => 0.0,
    }
}

/// Returns the sinus value of an angle described by a key that is a
/// multiple of pi/3.
///
/// Here, k2 = sin(pi/3)
fn sinus(key: u8) -> f64 {
    let k2 = 3.0_f64.sqrt() / 2.0;
    match key {
        0 => 0.0,
        1 => k2,
        2 => k2,
        3 => 0.0,
        4 => -k2,
        5 => -k2,
        _ => 0.0,
    }
}

fn rotate_and_cast(x: &[f64], y: &[f64], angle: f64) -> Vec<(f32, f32)> {
    let (sin, cos) = angle.sin_cos();
    x.iter()
        .zip(y)
        .map(|(x, y)| ((cos * x - sin * y) as f32, (sin * x + cos * y) as f32))
        .collect()
}

fn main() -> Result<(), Error> {
    let maximum_number_of_levels = 7;
    let levels = create_gosper_fractal(maximum_number_of_levels);
    for level in levels {
        level.plot()?;
    }
    Ok(())
}
