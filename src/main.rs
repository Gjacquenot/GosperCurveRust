use plotters::prelude::*;

#[derive(Debug)]
enum SegmentType {
    Type1,
    Type2,
}

#[derive(Debug)]
struct Level {
    scale: f64,
    directions: Vec<u8>,
    types: Vec<SegmentType>
}


fn create_initial_level() -> Level
{
    Level{scale: 7.0_f64.sqrt() / 2.0_f64,
          directions: vec![0_u8],
          types: vec![SegmentType::Type1]
        }
}

fn create_the_vector_of_levels() -> Vec<Level>
{
    let mut levels = Vec::<Level>::new();
    let initial_level = create_initial_level();
    levels.push(initial_level);
    levels
}

fn create_new_level(source_level: &Level) -> Level
{
    // t1 = 'abbaaab'
    let d1: Vec<u8> = vec![0_u8, 5_u8, 3_u8, 4_u8, 0_u8, 0_u8, 1_u8];

    // t2 = 'abbbaab'
    let d2: Vec<u8> = vec![1_u8, 0_u8, 0_u8, 4_u8, 3_u8, 5_u8, 0_u8];

    let mut types = Vec::<SegmentType>::new();
    let mut directions = Vec::<u8>::new();
    let n = source_level.directions.len();
    for i in 0..n {
        for j in 0..7 {
            match source_level.types[i] {
                SegmentType::Type1 => {
                    match j {
                        0 | 3 | 4 | 5 => {types.push(SegmentType::Type1);}
                        1 | 2 | 6 => {types.push(SegmentType::Type2);}
                        _ => {},
                    }
                    directions.push((source_level.directions[i] + d1[j])%6_u8);
                }
                SegmentType::Type2 => {
                    match j {
                        0 | 4 | 5 => {types.push(SegmentType::Type1);}
                        1 | 2 | 3 | 6 => {types.push(SegmentType::Type2);}
                        _ => {},
                    }
                    directions.push((source_level.directions[i] + d2[j])%6_u8);
                }
            }
        }
    }
    Level{scale: source_level.scale / 7.0_f64.sqrt(),
        directions: directions,
        types: types
      }
}

fn append_new_level(levels: &mut Vec<Level>)
{
    if let Some(last_level) = levels.last()
    {
        levels.push(create_new_level(last_level));
    }
}

fn cosinus(key: u8) -> f64
{
    // k1 = cos(pi/3)
    // k2 = sin(pi/3)
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

fn sinus(key: u8) -> f64
{
    // k1 = cos(pi/3)
    // k2 = sin(pi/3)
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

fn generate_level(level: &Level) -> Vec<(f32, f32)>
{
    // convert the formal description of a level to a x, y curve
    let scale = level.scale;
    let n = level.directions.len();
    let mut x: Vec<f64> = vec![0.0_f64; n + 1];
    let mut y: Vec<f64> = vec![0.0_f64; n + 1];
    let mut res: Vec<(f32, f32)>= vec![(0.0_f32, 0.0_f32); n + 1];
    for i in 0..n
    {
        x[i + 1] = x[i] + scale * cosinus(level.directions[i]);
        y[i + 1] = y[i] + scale * sinus(level.directions[i]);
    }
    for i in 0..(n+1)
    {
        res[i]= (x[i] as f32, y[i] as f32);
    }
    res
}

fn create_gosper_fractal(max_level: u8) -> Vec<Level>
{
    let mut levels = create_the_vector_of_levels();
    for _i in 1..max_level {
        append_new_level(&mut levels);
    }
    levels
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-2f32..2f32, -2f32..2f32)?;

    chart.configure_mesh().draw()?;

    let levels = create_gosper_fractal(5_u8);
    if let Some(last_level) = levels.last()
    {
        chart
        .draw_series(LineSeries::new(generate_level(last_level),
            &RED,
        ))?
        .label("y = x^2");
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}