use std::f32::consts::PI;

use super::pause::Pause;
use super::Scene;
use crate::audio::play_sfx;
use crate::consts::VIRTUAL_HEIGHT;
use crate::consts::VIRTUAL_WIDTH;
use crate::context::Context;
use crate::input::action_down;
use crate::input::action_pressed;
use crate::input::Action;
use crate::math::Ray;
use crate::math::Segment;
use macroquad::color::*;
use macroquad::math::Vec2;
use macroquad::rand;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;

pub struct Gameplay {
    pause_subscene: Pause,

    player_position: Vec2,
    walls: Vec<Segment>,
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

        self._player_movement(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.draw(ctx);
        } else {
            self._draw_scene();
        }
    }
}

impl Gameplay {
    pub async fn new(ctx: &mut Context) -> Self {
        let pause_subscene = Pause::new(ctx);
        let player_position = Vec2::new(300., 300.);

        let mut walls = vec![
            // outer walls
            Segment {
                src: Vec2::new(0., 0.),
                dst: Vec2::new(0., VIRTUAL_HEIGHT),
            },
            Segment {
                src: Vec2::new(0., 0.),
                dst: Vec2::new(VIRTUAL_WIDTH, 0.),
            },
            Segment {
                src: Vec2::new(VIRTUAL_WIDTH, 0.),
                dst: Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            },
            Segment {
                src: Vec2::new(0., VIRTUAL_HEIGHT),
                dst: Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            },
            // inner obstacles
            Segment {
                src: Vec2::new(0., 100.),
                dst: Vec2::new(100., 100.),
            },
            Segment {
                src: Vec2::new(100., 0.),
                dst: Vec2::new(100., 100.),
            },
            Segment {
                src: Vec2::new(300., 100.),
                dst: Vec2::new(400., 100.),
            },
            Segment {
                src: Vec2::new(400., 300.),
                dst: Vec2::new(450., 400.),
            },
            Segment {
                src: Vec2::new(500., 300.),
                dst: Vec2::new(590., 200.),
            },
        ];

        // TODO: How to seed? It's giving same results every time.
        let gen_uniform = || rand::rand() as f32 / u32::MAX as f32;
        for _ in 0..5 {
            walls.push(Segment {
                src: Vec2::new(
                    gen_uniform() * VIRTUAL_WIDTH,
                    gen_uniform() * VIRTUAL_HEIGHT,
                ),
                dst: Vec2::new(
                    gen_uniform() * VIRTUAL_WIDTH,
                    gen_uniform() * VIRTUAL_HEIGHT,
                ),
            })
        }

        Self {
            pause_subscene,
            player_position,
            walls,
        }
    }

    fn _draw_scene(&mut self) {
        // Draw light source
        let source = self.player_position;
        draw_circle(source.x, source.y, 10., WHITE);
        // draw_texture(&ctx.textures.example, 400., 300., WHITE);
        // Draw lines from light source to corners of squares

        // Create rays, sweeping over 360 degrees
        // TODO: Replace with only rays pointed at each intersection point in scene
        let mut rays = vec![];
        let num_rays = 360;
        for idx in 0..num_rays {
            let ratio = idx as f32 / num_rays as f32;
            let angle = ratio * 2. * PI;
            rays.push(Ray {
                origin: source,
                dir: Vec2::new(angle.cos(), angle.sin()),
            })
        }

        // draw walls
        for w in &self.walls {
            draw_line(w.src.x, w.src.y, w.dst.x, w.dst.y, 4., BLUE);
        }

        // find intersections
        let mut intersections = vec![];
        for ray in &rays {
            let mut closest_intersection: Option<Vec2> = None;
            for w in &self.walls {
                if let Some(new_intersection) = ray.intersection(w) {
                    if let Some(existing_int) = closest_intersection {
                        if ray.origin.distance(existing_int)
                            >= ray.origin.distance(new_intersection)
                        {
                            closest_intersection = Some(new_intersection);
                        }
                    } else {
                        closest_intersection = Some(new_intersection);
                    }
                }
            }

            // only keep the nearest one
            if let Some(intersection) = closest_intersection {
                intersections.push(intersection);
            }
        }

        // draw rays and interactions
        for intersection in intersections {
            draw_line(
                source.x,
                source.y,
                intersection.x,
                intersection.y,
                1.,
                // WHITE,
                Color::new(1.00, 1.00, 1.00, 0.5),
            );
            draw_circle(intersection.x, intersection.y, 2., ORANGE);
        }
    }

    fn _player_movement(&mut self, ctx: &mut Context) {
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
}
