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

fn create_initial_level() -> Level {
    Level {
        id: 0_u8,
        scale: 7.0_f64.sqrt() / 2.0,
        directions: vec![0],
        types: vec![SegmentType::Type1],
    }
}

fn create_the_vector_of_levels() -> Vec<Level> {
    vec![create_initial_level()]
}

/// Create a new level from an existing one.
///
/// This new level will have seven times more segment than its parent
/// level.
fn create_new_level(source_level: &Level) -> Level {
    // Each segment will be replaced by seven new ones
    // with directions depending on the type of their
    // parent.

    // New directions for a segment of type 1
    // t1 = 'abbaaab'
    let d1 = vec![0u8, 5, 3, 4, 0, 0, 1];

    // New directions for a segment of type 2
    // t2 = 'abbbaab'
    let d2 = vec![1u8, 0, 0, 4, 3, 5, 0];

    let mut new_level_types = Vec::new();
    let mut new_level_directions = Vec::new();
    let n = source_level.directions.len();
    for i in 0..n {
        for j in 0..7 {
            match source_level.types[i] {
                SegmentType::Type1 => {
                    match j {
                        0 | 3 | 4 | 5 => {
                            new_level_types.push(SegmentType::Type1);
                        }
                        1 | 2 | 6 => {
                            new_level_types.push(SegmentType::Type2);
                        }
                        _ => {}
                    }
                    new_level_directions.push((source_level.directions[i] + d1[j]) % 6);
                }
                SegmentType::Type2 => {
                    match j {
                        0 | 4 | 5 => {
                            new_level_types.push(SegmentType::Type1);
                        }
                        1 | 2 | 3 | 6 => {
                            new_level_types.push(SegmentType::Type2);
                        }
                        _ => {}
                    }
                    new_level_directions.push((source_level.directions[i] + d2[j]) % 6);
                }
            }
        }
    }
    Level {
        id: source_level.id + 1,
        scale: source_level.scale / 7.0_f64.sqrt(),
        directions: new_level_directions,
        types: new_level_types,
    }
}

fn append_new_level(levels: &mut Vec<Level>) {
    if let Some(last_level) = levels.last() {
        let new_level = create_new_level(last_level);
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
    let n = x.len();
    let mut res = vec![(0.0_f32, 0.0_f32); n];
    let cos_alpha = angle.cos();
    let sin_alpha = angle.sin();
    for i in 0..n {
        res[i] = (
            (cos_alpha * x[i] - sin_alpha * y[i]) as f32,
            (sin_alpha * x[i] + cos_alpha * y[i]) as f32,
        );
    }
    res
}

/// Convert the formal description of a level to a x, y curve.
///
/// Result is casted to a vector of tuples of float32 so that
/// it can directly be used by the plotting library
fn generate_level(level: &Level) -> Vec<(f32, f32)> {
    let scale = level.scale;
    let n = level.directions.len();
    let mut x = vec![0.0_f64; n + 1];
    let mut y = vec![0.0_f64; n + 1];
    for i in 0..n {
        x[i + 1] = x[i] + scale * cosinus(level.directions[i]);
        y[i + 1] = y[i] + scale * sinus(level.directions[i]);
    }

    let index = level.id as f64;
    let rotation_angle = index * ((3.0_f64.sqrt() / 5.0).atan());
    rotate_and_cast(&x, &y, rotation_angle)
}

fn plot_level(level: &Level) -> Result<(), Error> {
    let filename = format!("{}.png", level.id.to_string());
    let root = BitMapBackend::new(&filename, (1280, 1280)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(1)
        .build_ranged(-0.25..(7.0f32.sqrt() / 2.0 + 0.25), -1.25f32..0.75)?;

    chart.draw_series(LineSeries::new(generate_level(level), &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .draw()?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let maximum_number_of_levels = 7;
    let levels = create_gosper_fractal(maximum_number_of_levels);
    for level in levels {
        plot_level(&level)?;
    }
    Ok(())
}
