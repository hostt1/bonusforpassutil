// There are some key paths:
// 1. For USER
//   - /frineds/*.png
//   - avatar.png
// 2. For DEVELOPER
//   - /backgrounds/*.png
//   - /banners/*.png
//   - /badges/*.png
//   - /frames/*.png

use piet::kurbo::Size;
use piet::RenderContext;

mod components;

use components::{
    draw_avatar, draw_background, draw_badges, draw_banners, draw_frame, draw_friends,
    draw_text_blobs, Config,
};

pub const SIZE: Size = Size::new(3200., 1800.);

use std::path::Path;

use piet_common::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = SIZE;
    let scale = 2.0;
    let save_path = Path::new("res.png");

    let config = make_config()?;

    let s = serde_json::to_string(&config)?;
    println!("{s}");

    make_image(size, scale, save_path, config)
}

fn make_config() -> Result<Config, Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string("config.json")?;
    Ok(serde_json::from_str(&s)?)
}

fn make_image(
    size: Size,
    scale: f64,
    save_path: &Path,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::new()?;
    let mut target = device.bitmap_target(size.width as usize, size.height as usize, scale)?;
    let mut piet_context = target.render_context();

    draw(&mut piet_context, config)?;

    piet_context.finish()?;
    std::mem::drop(piet_context);

    target.save_to_file(save_path).map_err(Into::into)
}

fn draw(rc: &mut impl RenderContext, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    draw_background(rc, &config.background)?;

    draw_badges(rc, &config.main_badges, &config.badges)?;

    draw_banners(rc, &config.banners)?;

    draw_text_blobs(rc, &config.nickname, &config.entry_time, &config.about)?;

    let _ = draw_avatar(rc);

    let _ = draw_friends(rc);

    draw_frame(rc, &config.frame)?;

    Ok(())
}
