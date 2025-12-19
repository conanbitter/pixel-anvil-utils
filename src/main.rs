use std::path::Path;

use getargs::Opt;
use image::{ImageReader, RgbaImage};

use crate::{
    color::Color16,
    converters::{convert_fs, convert_ordered4, convert_ordered8, convert_posterize},
    plane::Plane,
};

mod color;
mod converters;
mod plane;

type DitheringMethod = fn(&RgbaImage) -> Plane<Color16>;

fn image_convert<P: AsRef<Path>>(input: P, output: P, dithering: DitheringMethod) -> anyhow::Result<()> {
    let img = ImageReader::open(input)?.decode()?.to_rgba8();
    let res_img = dithering(&img);
    res_img.save(output)?;

    Ok(())
}

fn choose_dithering(param: &str) -> DitheringMethod {
    match param {
        "none" => convert_posterize,
        "fs" => convert_fs,
        "ord4" => convert_ordered4,
        "ord8" => convert_ordered8,
        _ => convert_posterize,
    }
}

fn command_img<'arg, I: Iterator<Item = &'arg str>>(opts: &mut getargs::Options<&'arg str, I>) -> anyhow::Result<()> {
    let mut dithering: DitheringMethod = convert_posterize;
    let mut output = Path::new("");

    while let Some(opt) = opts.next_opt().unwrap() {
        match opt {
            Opt::Short('d') | Opt::Long("dither") => {
                dithering = choose_dithering(opts.value().unwrap());
            }
            Opt::Short('o') | Opt::Long("output") => {
                output = Path::new(opts.value().unwrap());
            }
            _ => {}
        }
    }

    println!("dith: {:?}, out: {:?}", dithering, output);

    for pos in opts.positionals() {
        println!("positional arg: {pos}");
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = wild::args().skip(1).collect::<Vec<_>>();
    let mut opts = getargs::Options::new(args.iter().map(String::as_str));

    let command = opts.next_positional();

    match command {
        None => println!("No command"),
        Some("img") => command_img(&mut opts)?,
        Some(_) => println!("Unknown command"),
    }

    //image_convert("assets/rainbow.png", "assets/rainbow.img", convert_ordered4)?;

    Ok(())
}
