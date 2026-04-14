use color_eyre::eyre::{self, Context, bail};

pub fn render_image(image_path: &str) -> eyre::Result<()> {
    let mut std_out = std::io::stdout();

    // Get terminal size;
    let (term_width, term_height) =
        crossterm::terminal::size().wrap_err("Failed to get terminal size")?;

    Ok(())
}
