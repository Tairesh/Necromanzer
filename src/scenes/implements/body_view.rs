use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        Rectangle,
    },
    Context, Event,
};

use crate::{
    app::App,
    colors::Colors,
    game::{
        bodies::Freshness,
        map::{item::ItemView, pos::TilePos},
    },
    geometry::Vec2,
    ui::{
        Alert, Horizontal, HoverableMesh, ItemDisplay, Label, Position, Positionate, SomeUISprites,
        SomeUISpritesMut, UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{back_btn, easy_back},
    SceneImpl, SomeTransitions,
};

pub struct BodyView {
    sprites: Vec<Box<dyn UiSprite>>,
}

impl BodyView {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(unit_id: usize, app: &App, ctx: &mut Context) -> Self {
        let world = app.world.as_ref().unwrap().borrow();
        let avatar = world.get_unit(unit_id);
        let window_size = app.window_size;
        let alert = Box::new(Alert::new(
            window_size.0 as f32,
            window_size.1 as f32,
            app.assets.alert.clone(),
            Position::by_left_top(0.0, 0.0),
        ));
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowBottomByBottom { offset: -20.0 },
            },
            &app.assets,
        );
        let mut name = Label::new(
            avatar.person().mind.name.clone(),
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_GOLDEN_ROD_YELLOW,
            Position::by_left_top(20.0, 20.0),
        );
        let name_size = name.calc_size(ctx);
        let gender = Label::new(
            format!(
                "({}, {})",
                avatar.person().age_name(),
                avatar.person().appearance.age
            ),
            app.assets.fonts.header2.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(30.0 + name_size.x, 20.0),
        );
        let wear = Label::new(
            "Wear:",
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_right_top(-20.0, 50.0),
        );
        let body = Label::new(
            "Body:",
            app.assets.fonts.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_left_top(20.0, 50.0),
        );
        let mut sprites: Vec<Box<dyn UiSprite>> = Vec::with_capacity(avatar.body.wear.len() + 4);
        sprites.push(alert);
        sprites.push(Box::new(name));
        sprites.push(Box::new(gender));
        sprites.push(Box::new(wear));
        sprites.push(Box::new(body));
        sprites.push(back_btn);
        let mut y = 0;
        avatar.body.wear.iter().for_each(|i| {
            y += 35;
            let mut disp = ItemDisplay::new(
                Some(i),
                app.assets.fonts.default2.clone(),
                Colors::LIGHT_GRAY,
                app.assets.tileset.clone(),
                Vec2::new(2.0, 2.0),
                Position::by_right_top(-20.0, 60.0 + y as f32),
            );
            let size = disp.calc_size(ctx) + Vec2::new(10.0, 8.0);
            sprites.push(Box::new(HoverableMesh::new(
                Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(0.0, 0.0, size.x, size.y),
                )
                .unwrap(),
                Colors::TRANSPARENT,
                Colors::WHITE_SMOKE.with_alpha(0.2),
                size,
                Position::by_right_top(-15.0, 57.0 + y as f32),
            )));
            sprites.push(Box::new(disp));
        });
        let mut y = 0;
        if let Some(item) = avatar.body.parts.get(&TilePos::new(0, 0)) {
            y += 35;
            let mut name = item.name.clone();
            name.push(':');
            sprites.push(Box::new(Label::new(
                name,
                app.assets.fonts.default2.clone(),
                Colors::LIGHT_GRAY,
                Position::by_left_top(20.0, 60.0 + y as f32),
            )));
            let color = match item.freshness() {
                Freshness::Fresh => Colors::LIGHT_PINK,
                Freshness::Rotten => Colors::LIME_GREEN,
                Freshness::Skeletal => Colors::WARM_IVORY,
            };
            let mut bp = Label::new(
                item.name(),
                app.assets.fonts.default2.clone(),
                color,
                Position::by_left_top(170.0, 60.0 + y as f32),
            );
            let size = bp.calc_size(ctx) + Vec2::new(10.0, 6.0);
            sprites.push(Box::new(HoverableMesh::new(
                Mesh::rectangle(
                    ctx,
                    ShapeStyle::Fill,
                    Rectangle::new(0.0, 0.0, size.x, size.y),
                )
                .unwrap(),
                Colors::TRANSPARENT,
                Colors::WHITE_SMOKE.with_alpha(0.2),
                size,
                Position::by_left_top(165.0, 57.0 + y as f32),
            )));
            sprites.push(Box::new(bp));
        }
        Self { sprites }
    }

    fn alert(&mut self) -> &mut Alert {
        self.sprites[0].as_alert().unwrap()
    }
}

impl SceneImpl for BodyView {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, false)
    }

    fn on_resize(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.alert().set_size(ctx, window_size);
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }
}
