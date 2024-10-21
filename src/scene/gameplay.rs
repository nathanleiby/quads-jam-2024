use super::pause::Pause;
use super::Scene;
use crate::audio::play_sfx;
use crate::consts::VIRTUAL_HEIGHT;
use crate::consts::VIRTUAL_WIDTH;
use crate::context::Context;
use crate::input::action_down;
use crate::input::action_pressed;
use crate::input::Action;
use macroquad::color::GREEN;
use macroquad::color::RED;
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::miniquad::window::screen_size;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_line;
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_frame_time;

pub struct Gameplay {
    pause_subscene: Pause,

    player_position: Vec2,
}

const MOVEMENT_SPEED: f32 = 300.;

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.update(ctx);
            // return; // TODO
        } else if action_pressed(Action::Pause, &ctx.gamepads) {
            self.pause_subscene.active = true;
            play_sfx(ctx, &ctx.audio.sfx.menu_select);
        }

        let mut movement_vec = Vec2::new(0., 0.);
        if action_down(Action::Up, &ctx.gamepads) {
            movement_vec.y += -1.;
        }
        if action_down(Action::Down, &ctx.gamepads) {
            movement_vec.y += 1.;
        }
        if action_down(Action::Left, &ctx.gamepads) {
            movement_vec.x += -1.;
        }
        if action_down(Action::Right, &ctx.gamepads) {
            movement_vec.x += 1.;
        }

        let delta = get_frame_time();
        if movement_vec != Vec2::new(0., 0.) {
            self.player_position += MOVEMENT_SPEED * delta * movement_vec.normalize();
            self.player_position = self
                .player_position
                .clamp(Vec2::default(), Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.draw(ctx);
        } else {
            // Draw obstacles
            draw_rectangle(0., 0., 100., 100., GREEN);
            draw_rectangle(0., 0., 100., 100., GREEN);
            // Draw light source
            let source = self.player_position;
            draw_circle(source.x, source.y, 10., WHITE);
            // draw_texture(&ctx.textures.example, 400., 300., WHITE);
            // Draw lines from light source to corners of squares

            draw_line(source.x, source.y, 100., 100., 1., RED);

            // draw_text(
            //     ctx,
            //     "Gameplay!",
            //     100.,
            //     100.,
            //     crate::text::Size::Medium,
            //     WHITE,
            // );
        }
    }
}

impl Gameplay {
    pub async fn new(ctx: &mut Context) -> Self {
        let pause_subscene = Pause::new(ctx);
        let player_position = Vec2::new(300., 300.);
        Self {
            pause_subscene,
            player_position,
        }
    }
}
