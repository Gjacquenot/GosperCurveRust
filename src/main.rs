use plotters::prelude::*;

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
        scale: 7.0_f64.sqrt() / 2.0_f64,
        directions: vec![0_u8],
        types: vec![SegmentType::Type1],
    }
}

fn create_the_vector_of_levels() -> Vec<Level> {
    vec![create_initial_level()]
}

fn create_new_level(source_level: &Level) -> Level {
    // t1 = 'abbaaab'
    let d1: Vec<u8> = vec![0, 5, 3, 4, 0, 0, 1];

    // t2 = 'abbbaab'
    let d2: Vec<u8> = vec![1, 0, 0, 4, 3, 5, 0];

    let mut new_level_types = Vec::<SegmentType>::new();
    let mut new_level_directions = Vec::<u8>::new();
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
                    new_level_directions.push((source_level.directions[i] + d1[j]) % 6_u8);
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
                    new_level_directions.push((source_level.directions[i] + d2[j]) % 6_u8);
                }
            }
        }
    }
    Level {
        id: source_level.id + 1_u8,
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
    for _i in 1..=max_level {
        append_new_level(&mut levels);
    }
    levels
}

fn cosinus(key: u8) -> f64 {
    // Returns the cosinus value of an angle described
    // by a key that is a multiple of pi/3.
    // Here, k1 = cos(pi/3)
    let k1 = 0.5_f64;
    match key {
        0_u8 => 1.0_f64,
        1_u8 => k1,
        2_u8 => -k1,
        3_u8 => -1.0_f64,
        4_u8 => -k1,
        5_u8 => k1,
        _ => 0.0_f64,
    }
}

fn sinus(key: u8) -> f64 {
    // Returns the sinus value of an angle described
    // by a key that is a multiple of pi/3.
    // Here, k2 = sin(pi/3)
    let k2 = 3.0_f64.sqrt() / 2.0_f64;
    match key {
        0_u8 => 0.0_f64,
        1_u8 => k2,
        2_u8 => k2,
        3_u8 => 0.0_f64,
        4_u8 => -k2,
        5_u8 => -k2,
        _ => 0.0_f64,
    }
}

fn generate_level(level: &Level) -> Vec<(f32, f32)> {
    // Convert the formal description of a level to a x, y curve.
    // Result is casted to a vector of tuples of float32 so that
    // it can directly be used by the plotting library
    let scale = level.scale;
    let n = level.directions.len();
    let mut x: Vec<f64> = vec![0.0_f64; n + 1];
    let mut y: Vec<f64> = vec![0.0_f64; n + 1];
    let mut res: Vec<(f32, f32)> = vec![(0.0_f32, 0.0_f32); n + 1];
    for i in 0..n {
        x[i + 1] = x[i] + scale * cosinus(level.directions[i]);
        y[i + 1] = y[i] + scale * sinus(level.directions[i]);
    }

    let index = level.id as f64;
    let alpha = index * ((3.0_f64.sqrt() / 5.0).atan());
    let cos_alpha = alpha.cos();
    let sin_alpha = alpha.sin();
    for i in 0..=n {
        res[i] = (
            (cos_alpha * x[i] - sin_alpha * y[i]) as f32,
            (sin_alpha * x[i] + cos_alpha * y[i]) as f32,
        );
    }
    res
}

fn plot_level(level: &Level) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.png", level.id.to_string());
    let root = BitMapBackend::new(&filename, (1280, 1280)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root).margin(1).build_ranged(
        -0.25f32..(7.0f32.sqrt() / 2.0_f32 + 0.25f32),
        -1.25f32..0.75f32,
    )?;

    chart.draw_series(LineSeries::new(generate_level(level), &BLACK))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .draw()?;
    Ok(())
}

fn main() {
    let maximum_number_of_levels = 7_u8;
    let levels = create_gosper_fractal(maximum_number_of_levels);
    for level in levels {
        let _result = plot_level(&level);
    }
}
