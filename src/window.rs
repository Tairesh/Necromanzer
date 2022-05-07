use settings::window::WindowSettings;
use tetra::graphics::ImageData;
use tetra::window::WindowPosition;
use tetra::{window, Context, ContextBuilder};

pub fn create_context<S: Into<String>>(
    title: S,
    settings: &WindowSettings,
) -> tetra::Result<Context> {
    let mut ctx_builder = ContextBuilder::new(title, settings.width, settings.height);
    ctx_builder
        .show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    let mut ctx = ctx_builder.build()?;

    let mut icon = ImageData::from_encoded(include_bytes!("../assets/img/zombie.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;
    if settings.fullscreen {
        window::set_fullscreen(&mut ctx, true).ok();
    } else {
        let current_monitor = window::get_current_monitor(&ctx).unwrap_or(0);
        window::set_position(
            &mut ctx,
            WindowPosition::Centered(current_monitor),
            WindowPosition::Centered(current_monitor),
        );
    }

    Ok(ctx)
}
