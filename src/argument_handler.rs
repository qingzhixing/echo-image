use color_eyre::eyre::{self, bail};

use crate::project_info;

#[derive(Debug, Default)]
pub struct EchoImageArguments {
    pub image_path: String,
    pub need_help: bool,
    pub need_version: bool,
}

pub fn parse_arguments(
    args: &mut impl Iterator<Item = String>,
) -> eyre::Result<EchoImageArguments> {
    let mut arguments = EchoImageArguments::default();

    let mut path_vec: Vec<String> = Vec::new();
    // 跳过程序名进行遍历
    for arg in args.skip(1) {
        match arg.as_str() {
            "-h" | "--help" => arguments.need_help = true,
            "-v" | "--version" => arguments.need_version = true,
            arg if arg.starts_with('-') => bail!("Unknown option: {}", arg),
            path => path_vec.push(path.to_string()),
        }
    }

    // write image path to arguments.image_path
    if (!arguments.need_help) && (!arguments.need_version) {
        match path_vec.len() {
            0 => bail!("No image path provided"),
            1 => arguments.image_path = path_vec[0].clone(),
            _ => bail!("Multiple image paths are not supported"),
        }
    }

    Ok(arguments)
}

pub fn print_help() {
    println!(
        "💭 Usage: {}: [image_path] [-h | --help] [-v | --version]",
        project_info::NAME
    );
    println!("🌈 Options:");
    println!("  -h, --help  Show this help message");
    println!("  -v, --version Show version version");
    println!("  image_path  Path to the image file");
}

pub fn print_version() {
    println!("🏁 {}: {}", project_info::NAME, project_info::DESCRIPTION);
    println!("✨ Author: {}", project_info::AUTHOR);
    println!("🔗 Github: {}", project_info::REPOSITORY);
}
