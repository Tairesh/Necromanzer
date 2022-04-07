use settings::window::WindowSettings;
use tetra::graphics::ImageData;
use tetra::window::WindowPosition;
use tetra::{window, Context, ContextBuilder};

pub fn create_context<S>(title: S, settings: &WindowSettings) -> tetra::Result<Context>
where
    S: Into<String>,
{
    let mut ctx_builder = ContextBuilder::new(title, settings.width, settings.height);
    ctx_builder
        .show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    if settings.fullscreen {
        ctx_builder.fullscreen(true);
    }
    let mut ctx = ctx_builder.build()?;

    let mut icon = ImageData::from_encoded(include_bytes!("../inc/img/zombie.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;
    window::set_position(
        &mut ctx,
        WindowPosition::Centered(0),
        WindowPosition::Centered(0),
    );

    Ok(ctx)
}
