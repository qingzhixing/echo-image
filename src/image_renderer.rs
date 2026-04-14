use std::path::Path;

use color_eyre::eyre::{self};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use image::ImageReader;
use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::Line,
    widgets::{Block, Borders},
};
use ratatui_image::{StatefulImage, picker::Picker};

pub fn render_image(image_path: &str) -> eyre::Result<()> {
    let picker = Picker::from_query_stdio()?;
    let image = ImageReader::open(image_path)?.decode()?;
    let mut protocol = picker.new_resize_protocol(image);

    ratatui::run(|terminal| -> eyre::Result<()> {
        loop {
            terminal.draw(|frame| {
                let centered_area = frame
                    .area()
                    .centered(Constraint::Ratio(4, 5), Constraint::Ratio(4, 5));
                let file_name = Path::new(image_path).file_name().unwrap().to_string_lossy();

                let block = Block::default()
                    .title(Line::from(file_name).centered())
                    .title_bottom(Line::from("Press Esc to exit").centered().yellow())
                    .borders(Borders::ALL);

                let image_area = block.inner(centered_area);

                frame.render_widget(block, centered_area);

                frame.render_stateful_widget(StatefulImage::new(), image_area, &mut protocol);
            })?;
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => return Ok(()),
                _ => continue,
            }
        }
    })?; // 传播 run 本身的错误
    Ok(())
}
