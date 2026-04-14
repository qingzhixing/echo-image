use std::{io::Write, path::Path, thread::sleep, time::Duration};

use color_eyre::eyre::{self, Context};
use crossterm::{
    cursor::MoveToNextLine,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use image::ImageReader;
use log::debug;
use ratatui::{
    Terminal,
    layout::Rect,
    prelude::CrosstermBackend,
    widgets::{Block, Borders},
};
use ratatui_image::{StatefulImage, picker::Picker};

pub fn render_image(image_path: &str) -> eyre::Result<()> {
    // 启用 raw mode（为了让 ratatui 正常工作，但实际不需要捕获输入）
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // 创建图片协议探测器, 用于决定用什么终端协议显示图片
    let picker =
        Picker::from_query_stdio().context("Failed to initialize terminal graphics protocol")?;

    // 加载并解码图片
    let dyn_img = ImageReader::open(image_path)
        .context("Failed to open image file")?
        .decode()
        .context("Failed to decode image")?;

    // 创建一个可以根据渲染区域自动调整大小的协议
    let mut protocol = picker.new_resize_protocol(dyn_img);

    // 获取文件名作为 Block 标题
    let file_name = Path::new(image_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // render
    terminal.draw(|frame| {
        // 计算一个合适的渲染区域（Rect）
        let frame_area = frame.area();
        let render_area = Rect {
            x: frame_area.x,
            y: frame_area.y,
            width: frame_area.width / 4,
            height: frame_area.height / 4,
        };

        debug!("Frame area: {:?}", frame_area);
        debug!("Calculated render area: {:?}", render_area);

        let block = Block::default().borders(Borders::ALL).title(file_name);
        let inner_area = block.inner(render_area);

        frame.render_widget(block, render_area);

        // 在渲染区域内显示图片
        let image_widget = StatefulImage::new();
        frame.render_stateful_widget(image_widget, inner_area, &mut protocol);
    })?;

    std::io::stdout().flush()?;

    // 确保图形协议完成渲染
    sleep(Duration::from_millis(100));

    disable_raw_mode()?;

    // 9. 将光标移动到图片的下一行（行首）
    execute!(std::io::stdout(), MoveToNextLine(1))?;
    std::io::stdout().flush()?;

    Ok(())
}
