use assets::Assets;
use colors::Colors;
use direction::Direction;
use geometry::DIR9;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::alert::Alert;
use sprites::button::Button;
use sprites::label::Label;
use sprites::meshy::HoverableMesh;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, MouseButton};
use tetra::Context;
use world::World;
use {input, Vec2};

pub struct Wielding {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    world: Rc<RefCell<World>>,
}

impl Wielding {
    pub fn new(assets: Rc<RefCell<Assets>>, world: Rc<RefCell<World>>, ctx: &mut Context) -> Self {
        let mut items = Vec::new();
        for (dx, dy) in DIR9 {
            let pos = world.borrow().avatar.pos.add_delta(dx, dy);
            for (i, item) in world.borrow_mut().load_tile(pos).items.iter().enumerate() {
                let dir = Direction::from((dx, dy));
                items.push((
                    format!("{}:{}", dir.as_str(), i),
                    dir,
                    item.region(),
                    item.name().to_string(),
                ));
            }
        }
        let height = 33.0 + items.len() as f32 * 50.0;
        let mut y = -height / 2.0;
        let mut sprites: Vec<Rc<RefCell<dyn Sprite>>> = vec![Rc::new(RefCell::new(Alert::new(
            300.0,
            height,
            assets.clone(),
            Position::center(),
        )))];
        for (i, (btn_id, dir, rect, name)) in items.into_iter().enumerate() {
            sprites.push(Rc::new(RefCell::new(HoverableMesh::new(
                Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 260.0, 50.0))
                    .unwrap(),
                if i % 2 == 1 {
                    Colors::DARK_GRAY.with_alpha(0.3)
                } else {
                    Colors::TRANSPARENT
                },
                Colors::KHAKI.with_alpha(0.6),
                Vec2::new(260.0, 50.0),
                Position {
                    x: Horizontal::AtWindowCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y + 18.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                name.as_str(),
                assets.borrow().default2.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y + 20.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                dir.as_str(),
                assets.borrow().default.clone(),
                Colors::LIGHT_GRAY,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -70.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y + 40.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Button::icon(
                btn_id.as_str(),
                vec![],
                rect,
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -120.0 },
                    y: Vertical::AtWindowCenter {
                        offset: y + 18.0 + 25.0,
                    },
                },
            ))));
            y += 50.0;
        }
        Self { sprites, world }
    }
}

impl Scene for Wielding {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        let mut parts = btn_id.split(':');
        match (parts.next(), parts.next()) {
            (Some(dir), Some(i)) => {
                let dir = Direction::from(dir);
                let i: usize = i.parse().unwrap();
                let pos = self.world.borrow().avatar.pos.add(dir);
                let mut world = self.world.borrow_mut();
                let item = world.load_tile(pos).items.get(i).unwrap().clone();
                world.load_tile_mut(pos).items.remove(i);
                world.avatar.wield.push(item);
                // TODO: provide some way to add msg to log
                Some(Transition::Pop)
            }
            (_, _) => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape)
            || input::is_mouse_button_pressed(ctx, MouseButton::X1)
        {
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
