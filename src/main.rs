use color_eyre::eyre::{self};

mod argument_handler;
mod image_renderer;
mod project_info;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let arguments = match argument_handler::parse_arguments(&mut std::env::args()) {
        Ok(arguments) => arguments,
        Err(err) => {
            argument_handler::print_help();
            return Err(err);
        }
    };
    if arguments.need_version {
        argument_handler::print_version();
        return Ok(());
    }
    if arguments.need_help {
        argument_handler::print_help();
        return Ok(());
    }

    image_renderer::render_image(&arguments.image_path)?;

    Ok(())
}
