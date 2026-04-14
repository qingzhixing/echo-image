use color_eyre::eyre::{self, Context};
use image::ImageReader;
use log::debug;
use ratatui::layout::Rect;
use ratatui_image::picker::Picker;

pub fn render_image(image_path: &str) -> eyre::Result<()> {
    let mut std_out = std::io::stdout();

    // Get terminal size;
    let (term_width, term_height) =
        crossterm::terminal::size().wrap_err("Failed to get terminal size")?;

    debug!("Terminal size: {}x{}", term_width, term_height);

    // 创建图片协议探测器, 用于决定用什么终端协议显示图片
    let mut picker =
        Picker::from_query_stdio().context("Failed to initialize terminal graphics protocol")?;

    // 加载并解码图片
    let dyn_img = ImageReader::open(image_path)
        .context("Failed to open image file")?
        .decode()
        .context("Failed to decode image")?;

    // 创建一个可以根据渲染区域自动调整大小的协议
    let mut protocol = picker.new_resize_protocol(dyn_img);

    // 计算一个合适的渲染区域（Rect）
    let render_area = Rect {
        x: 0,
        y: 0,
        width: term_width / 4,
        height: term_height / 4,
    };

    Ok(())
}
