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


// res = {0: {'s': 7.0**0.5, 't': ['a'], 'd': [0]}}

fn function2(d1: &mut Vec<i32>)
{
    fn f_add_modulo6 (m: i32, d: i32) -> i32 { m + d } // [(m + e) % 6 for e in d]
    for x in d1 {
        *x = f_add_modulo6(42, *x);
    }
}

fn create_gosper_fractal(max_level: u8)
{
    // Segment type and directions for pattern 1
    let t1 = "abbaaab";
    let mut d1 = vec![0, 5, 3, 4, 0, 0, 1];

    // Segment type and directions for pattern 2
    let t2 = "abbbaab";
    let d2 = vec![1, 0, 0, 4, 3, 5, 0];

    // d1.push(4);
    // d2.push(4);
    // vec.push(4);

    //let mut res = {0: {'s': 7.0**0.5,
    //                   't': ['a'],
    //                   'd': [0]}};

    let k1 = 0.5_f64;
    let k2 = 3.0_f64.sqrt() / 2.0_f64;
    println!(
        "Today, run for {} minutes!",k2
    );

    function2(&mut d1);

    for x in d1 {
        println!("{}", x);
    }

    // let k = 3;
    // match k {
    //     0 => 1.0,
    //     1 => k1,
    //     2 => -k1,
    //     3 => -1.0,
    //     4 => -k1,
    //     5 => +k1
    // }
}


fn main2() {
    println!("Hello, world!");
    create_gosper_fractal(6);

    println!("{:?}", create_initial_level());
    let mut levels = create_the_vector_of_levels();
    append_new_level(&mut levels);
    //append_new_level(&mut levels);
    //append_new_level(&mut levels);
    //append_new_level(&mut levels);
    println!("{:?}", levels);
    println!("{:?}", cosinus(2_u8));
    println!("{:?}", cosinus(4_u8));
    if let Some(last_level) = levels.last()
    {
        generate_level(last_level);
    }
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

    // chart
    //     .draw_series(LineSeries::new(
    //         (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
    //         &RED,
    //     ))?
    //     .label("y = x^2")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    let mut levels = create_the_vector_of_levels();
    append_new_level(&mut levels);
    append_new_level(&mut levels);
    append_new_level(&mut levels);
    append_new_level(&mut levels);
    append_new_level(&mut levels);
    // append_new_level(&mut levels);
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
    main2();
    Ok(())
}