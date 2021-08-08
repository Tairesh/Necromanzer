extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::surface::Surface;
use std::time::Duration;

const VERSION: &str = "v0.1.0";

fn render(
    canvas: &mut WindowCanvas,
    bg: &Texture,
    logo: &Texture,
    version_label: &Texture,
) -> Result<(), String> {
    canvas.clear();
    let (w, h) = canvas.output_size()?;
    let screen_center = (w as i32 / 2, h as i32 / 2);

    let bg_position = Some(Rect::new(
        screen_center.0 - bg.query().width as i32 / 2,
        screen_center.1 - bg.query().height as i32 / 2,
        bg.query().width,
        bg.query().height,
    ));
    canvas.copy(bg, None, bg_position)?;

    let logo_position = Some(Rect::new(
        screen_center.0 - logo.query().width as i32 / 2,
        10,
        logo.query().width,
        logo.query().height,
    ));
    canvas.copy(logo, None, logo_position)?;

    canvas.copy(
        version_label,
        None,
        Some(Rect::new(
            logo_position.unwrap().right() - version_label.query().width as i32 - 15,
            logo_position.unwrap().top() + 25,
            version_label.query().width,
            version_label.query().height,
        )),
    )?;

    canvas.present();
    Ok(())
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let display_mode = video_subsystem.current_display_mode(0).unwrap();
    // let mut window = video_subsystem.window("Necromanzer", display_mode.w as u32, display_mode.h as u32)
    // .fullscreen_desktop()
    let mut window = video_subsystem
        .window("Necromanzer", 1024, 768)
        .build()
        .unwrap();
    let icon: Surface = LoadSurface::from_file("res/img/zombie.png").unwrap();
    window.set_icon(icon);

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let bg = texture_creator.load_texture("res/img/bg.jpg").unwrap();
    let logo = texture_creator.load_texture("res/img/logo.png").unwrap();

    let font_context = sdl2::ttf::init().unwrap();
    let consolab = font_context
        .load_font("res/fonts/consolab.ttf", 16)
        .unwrap();
    let version_label = texture_creator
        .create_texture_from_surface(
            consolab
                .render(VERSION)
                .blended(Color::RGB(0, 0, 0))
                .unwrap(),
        )
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        render(&mut canvas, &bg, &logo, &version_label).expect("Render failed!");
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
