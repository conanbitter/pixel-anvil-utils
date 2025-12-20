use std::path::{Path, PathBuf};

use argh::FromArgs;
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

#[derive(FromArgs, Debug)]
/// Graphics conversion
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommand {
    Image(CommandImage),
}

#[derive(FromArgs, Debug)]
/// Image conversion
#[argh(subcommand, name = "img")]
struct CommandImage {
    #[argh(option, short = 'd', from_str_fn(choose_dithering), default = "convert_posterize")]
    /// image dithering method
    dither: DitheringMethod,

    #[argh(option, short = 'o')]
    /// output file name for single file or output folder for multiple files
    output: PathBuf,

    #[argh(positional)]
    files: Vec<PathBuf>,
}

fn choose_dithering(param: &str) -> Result<DitheringMethod, String> {
    match param {
        "none" => Ok(convert_posterize),
        "fs" => Ok(convert_fs),
        "ord4" => Ok(convert_ordered4),
        "ord8" => Ok(convert_ordered8),
        _ => Err(String::from("Unknown dithering method")),
    }
}

fn image_convert<P: AsRef<Path>>(input: P, output: P, dithering: DitheringMethod) -> anyhow::Result<()> {
    let img = ImageReader::open(input)?.decode()?.to_rgba8();
    let res_img = dithering(&img);
    res_img.save(output)?;

    Ok(())
}

fn command_img(opts: CommandImage) -> anyhow::Result<()> {
    if opts.files.is_empty() {
        eprintln!("No files.");
    } else if opts.files.len() == 1 {
        println!("Converting {}", opts.files[0].file_name().unwrap().to_str().unwrap());
        image_convert(&opts.files[0], &opts.output, opts.dither)?;
    } else {
        for file in opts.files {
            println!("Converting {}", file.file_name().unwrap().to_str().unwrap());
            let out_filename = opts.output.join(file.with_extension("img").file_name().unwrap());
            image_convert(&file, &out_filename, opts.dither)?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = wild::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(String::as_str).collect::<Vec<_>>();

    let args = Args::from_args(&["conv"], &args).unwrap_or_else(|early_exit| {
        std::process::exit(match early_exit.status {
            Ok(()) => {
                println!("{}", early_exit.output);
                0
            }
            Err(()) => {
                eprintln!("{}\nRun conv --help for more information.", early_exit.output);
                1
            }
        })
    });

    match args.subcommand {
        SubCommand::Image(opts) => {
            command_img(opts)?;
        }
    }

    println!("Done!");

    Ok(())
}
