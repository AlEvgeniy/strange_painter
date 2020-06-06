use serde::{Serialize, Deserialize};
use clap::{App, Arg};

mod palette;
pub use palette::Pal;

mod painter;
use painter::paint;

#[derive(Serialize, Deserialize)]
pub struct Transform
{
    cut: bool,
    numerator_coefs: Vec<(f32, f32)>,
    denominator_coefs: Vec<(f32, f32)>,
}

#[derive(Serialize, Deserialize)]
pub struct Input
{
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    transforms: Vec<Transform>,
}

fn main() -> Result<(), String>
{
    let matches = App::new("Strange painter")
    .arg(Arg::with_name("input")
    .short("i")
    .long("input")
    .value_name("INPUT FILE NAME")
    .takes_value(true)
    .required(true)
    .multiple(false)
    .help("Sets input file name"))
    .arg(Arg::with_name("output")
    .short("o")
    .long("output")
    .value_name("OUTPUT FILE NAME")
    .takes_value(true)
    .required(true)
    .multiple(false)
    .help("Sets output file name"))
    .arg(Arg::with_name("palette")
    .short("p")
    .long("palette")
    .value_name("PALETTE FILE NAME")
    .takes_value(true)
    .required(false)
    .multiple(false)
    .help("Sets image file name, where first scan line will be used as palette"))
    .arg(Arg::with_name("size")
    .short("s")
    .long("size")
    .value_name("WIDTH:HEIGHT")
    .takes_value(true)
    .required(true)
    .multiple(false)
    .help("Sets size of picture in pixels"))
    .arg(Arg::with_name("depth")
    .short("d")
    .long("depth")
    .value_name("NUMBER OF ITERATIONS")
    .takes_value(true)
    .required(false)
    .multiple(false)
    .default_value("50")
    .help("Sets number of iterations in painting"))
    .get_matches();

    let inp_name = matches.value_of("input").ok_or("input parameter not specified, WTF?")?;
    let out_name = matches.value_of("output").ok_or("output parameter not specified, WTF?")?;
    let sz = matches.value_of("size").ok_or("size parameter not specified, WTF?")?;
    let sizes: Vec<&str> = sz.split(":").collect();
    if sizes.len() != 2
    {
        return Err("Invalid sizes argument!".to_string());
    }
    let width: u32 = sizes[0].parse().map_err(|e| format!("Invalid width: {}", e))?;
    let height: u32 = sizes[1].parse().map_err(|e| format!("Invalid height: {}", e))?;

    let depth: usize = matches.value_of("depth").ok_or("depth parameter not specified, WTF?")?.parse().map_err(|e| format!("Invalid depth parameter: {}", e))?;
    
    let pal_name = matches.value_of("palette");
    let the_pal: Pal = Pal::new(pal_name)?;

    let inp: Input =
    {
        let f = std::fs::OpenOptions::new().read(true).write(false).create(false).create_new(false).truncate(false).open(inp_name).map_err(|e| format!("Can not open input file: {}", e))?;
        serde_yaml::from_reader(f).map_err(|e| format!("Invalid input file format: {}", e))?
    };

    if inp.left >= inp.right
    {
        return Err("Left field can not be greater then right!".to_string());
    }

    if inp.bottom >= inp.top
    {
        return Err("Bottom field can not be greater then top!".to_string());
    }

    let pic = paint(&inp, width as usize, height as usize, &the_pal, depth);

    image::save_buffer(out_name, &pic, width, height, image::ColorType::Rgb8).map_err(|e| format!("Can not save picture: {}", e))?;

    println!("DONE");
    Ok(())
}