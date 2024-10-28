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
use macroquad::math::Circle;
use macroquad::math::Rect;
use macroquad::math::Vec2;
use macroquad::rand::rand;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;
use macroquad::time::get_time;

pub struct Gameplay {
    pause_subscene: Pause,

    player_position: Vec2,
    walls: Vec<Segment>,
    asteroids: Vec<Asteroid>,
    last_spawn_asteroid_time: f64,
    bullets: Vec<Bullet>,
}

const MOVEMENT_SPEED: f32 = 300.;
const BULLET_RADIUS: f32 = 5.;
const BULLET_COLOR: Color = GREEN;
const BULLET_MOVEMENT_SPEED: f32 = 200.;
const PLAYER_RADIUS: f32 = 5.;

struct Bullet {
    circle: Circle,
}

struct Asteroid {
    rect: Rect,
}

impl Asteroid {
    fn to_segments(&self) -> Vec<Segment> {
        let r = self.rect;
        vec![
            Segment {
                src: Vec2::new(r.x, r.y),
                dst: Vec2::new(r.x + r.w, r.y),
            },
            Segment {
                src: Vec2::new(r.x, r.y),
                dst: Vec2::new(r.x, r.y + r.h),
            },
            Segment {
                src: Vec2::new(r.x, r.y + r.h),
                dst: Vec2::new(r.x + r.w, r.y + r.h),
            },
            Segment {
                src: Vec2::new(r.x + r.w, r.y),
                dst: Vec2::new(r.x + r.w, r.y + r.h),
            },
        ]
    }
}

impl Scene for Gameplay {
    fn update(&mut self, ctx: &mut Context) {
        if self.pause_subscene.active {
            self.pause_subscene.update(ctx);
            // return; // TODO
        } else if action_pressed(Action::Pause, &ctx.gamepads) {
            self.pause_subscene.active = true;
            play_sfx(ctx, &ctx.audio.sfx.menu_select);
        }

        self.player_movement(ctx);
        self.player_attack(ctx);
        self.asteroid_movement();

        let player_circle = Circle::new(
            self.player_position.x,
            self.player_position.y,
            PLAYER_RADIUS,
        );
        // check for collisions
        for a in &self.asteroids {
            if player_circle.overlaps_rect(&a.rect) {
                // TODO: Handle the collision
                println!("collision");
            }
        }

        let elapsed = get_time();
        if elapsed > self.last_spawn_asteroid_time + 1. {
            self.spawn_asteroid();
            self.last_spawn_asteroid_time = elapsed;
        }
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

        let asteroids = vec![Asteroid {
            rect: Rect::new(300., 600., 30., 30.),
        }];

        let mut walls = vec![
            // outer walls
            // Segment {
            //     // Left
            //     src: Vec2::new(0., 0.),
            //     dst: Vec2::new(0., VIRTUAL_HEIGHT),
            // },
            Segment {
                // Top
                src: Vec2::new(0., 0.),
                dst: Vec2::new(VIRTUAL_WIDTH, 0.),
            },
            // Segment {
            //     // Right
            //     src: Vec2::new(VIRTUAL_WIDTH, 0.),
            //     dst: Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            // },
            Segment {
                // Bottom
                src: Vec2::new(0., VIRTUAL_HEIGHT),
                dst: Vec2::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
            },
            //     // inner obstacles
            //     Segment {
            //         src: Vec2::new(0., 100.),
            //         dst: Vec2::new(100., 100.),
            //     },
            //     Segment {
            //         src: Vec2::new(100., 0.),
            //         dst: Vec2::new(100., 100.),
            //     },
            //     Segment {
            //         src: Vec2::new(300., 100.),
            //         dst: Vec2::new(400., 100.),
            //     },
            //     Segment {
            //         src: Vec2::new(400., 300.),
            //         dst: Vec2::new(450., 400.),
            //     },
            //     Segment {
            //         src: Vec2::new(500., 300.),
            //         dst: Vec2::new(590., 200.),
            //     },
        ];

        // TODO: How to seed? It's giving same results every time.
        // let gen_uniform = || rand::rand() as f32 / u32::MAX as f32;
        // for _ in 0..5 {
        //     walls.push(Segment {
        //         src: Vec2::new(
        //             gen_uniform() * VIRTUAL_WIDTH,
        //             gen_uniform() * VIRTUAL_HEIGHT,
        //         ),
        //         dst: Vec2::new(
        //             gen_uniform() * VIRTUAL_WIDTH,
        //             gen_uniform() * VIRTUAL_HEIGHT,
        //         ),
        //     })
        // }

        Self {
            pause_subscene,
            player_position,
            walls,
            asteroids,
            last_spawn_asteroid_time: -f64::INFINITY,
            bullets: vec![],
        }
    }

    fn _draw_scene(&mut self) {
        let source = self.player_position;
        // Draw lines from light source to corners of squares
        // Create rays, sweeping over 360 degrees
        // TODO: Replace with only rays pointed at each intersection point in scene
        let mut rays = vec![];
        // let num_rays = 60;
        let num_rays = 360;
        for idx in 0..num_rays {
            let ratio = idx as f32 / num_rays as f32;
            // let angle = ratio * (2. * PI / 6.);
            let angle = ratio * (2. * PI);
            rays.push(Ray {
                origin: source,
                dir: Vec2::new(angle.cos(), angle.sin()),
            })
        }

        let mut asteroids_segs = vec![];
        for a in &self.asteroids {
            for seg in a.to_segments() {
                asteroids_segs.push(seg);
            }
        }

        // draw walls
        for w in &self.walls {
            draw_line(w.src.x, w.src.y, w.dst.x, w.dst.y, 4., BLUE);
        }

        // for seg in &asteroids_segs {
        //     draw_line(seg.src.x, seg.src.y, seg.dst.x, seg.dst.y, 2., BROWN);
        // }

        let mut collideable = vec![];
        for w in &self.walls {
            collideable.push(w);
        }
        for s in &asteroids_segs {
            collideable.push(s);
        }

        // find intersections
        let mut intersections = vec![];
        for ray in &rays {
            let mut closest_intersection: Option<Vec2> = None;
            for w in &collideable {
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
            // intersection point
            draw_circle(intersection.x, intersection.y, 2., ORANGE);
        }

        // Draw Player (light source)
        draw_circle(source.x, source.y, PLAYER_RADIUS, WHITE);
        // draw_texture(&ctx.textures.example, 400., 300., WHITE);

        // draw bullet
        for b in &self.bullets {
            draw_circle(b.circle.x, b.circle.y, b.circle.r, BULLET_COLOR);
        }
    }

    fn player_movement(&mut self, ctx: &mut Context) {
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

    fn player_attack(&mut self, ctx: &mut Context) {
        let delta = get_frame_time();
        for b in &mut self.bullets {
            b.circle.x += BULLET_MOVEMENT_SPEED * delta;
        }
        // TODO: Add another action for "attack"
        if action_pressed(Action::Confirm, &ctx.gamepads) {
            self.bullets.push(Bullet {
                circle: Circle::new(
                    self.player_position.x + 10.,
                    self.player_position.y,
                    BULLET_RADIUS,
                ),
            });
        }
    }

    fn asteroid_movement(&mut self) {
        let asteroid_movement_speed = 50.;
        let delta = get_frame_time();
        for a in &mut self.asteroids {
            a.rect.x -= asteroid_movement_speed * delta;
        }
    }

    fn spawn_asteroid(&mut self) {
        // random y
        let ratio_y = rand() as f32 / u32::MAX as f32;
        // random height
        let ratio_h = rand() as f32 / u32::MAX as f32;

        self.asteroids.push(Asteroid {
            rect: Rect::new(
                VIRTUAL_WIDTH,
                (ratio_y * VIRTUAL_HEIGHT).floor(),
                30.,
                20. + (180. * ratio_h).floor(),
            ),
        })
    }
}
