use macroquad::math::Vec2;

struct Ray {
    origin: Vec2,
    dir: Vec2,
}

struct Segment {
    src: Vec2,
    dst: Vec2,
}

impl Segment {
    // parametric
    // Point + Direction * T
    fn parametric_dir(self) -> Vec2 {
        Vec2::new(self.dst.x - self.src.x, self.dst.y - self.src.y)
    }

    // fn slope(self) -> f32 {
    //     (self.dst.y - self.src.y) / (self.dst.x - self.src.x)
    // }
}

fn cross(v: Vec2, w: Vec2) -> f32 {
    v.x * w.y - v.y * w.x
}

impl Ray {
    // https://stackoverflow.com/a/565282/950683
    fn intersection(self, segment: Segment) -> (bool, Vec2) {
        let p = self.origin;
        let r = self.dir;

        let q = segment.src;
        let s = segment.parametric_dir();

        // intersect if p + tr = q + us, where t and u are "time" params
        // (p + t r) × s = (q + u s) × s
        // ->
        // t = (q − p) × s / (r × s)
        // u = (q − p) × r / (r × s)
        let r_x_s = cross(r, s);
        let t = cross(q - p, s) / r_x_s;
        let u = cross(q - p, r) / r_x_s;

        let check1 = r_x_s == 0.;
        let check2 = cross(q - p, r) == 0.;

        match (check1, check2) {
            (true, true) => {
                // the lines are collinear (overlapping)
                // TODO: Need to put actual intersectio
                return (false, Vec2::default());
            }
            (true, false) => {
                // parallel and non-intersecting
                return (false, Vec2::default());
            }
            (false, _) => {
                // if and 0 ≤ t ≤ 1 and 0 ≤ u ≤ 1, the two line segments meet at the point p + t r = q + u s.
                if 0. <= t && t <= 1. && 0. <= u && u <= 1. {
                    return (true, p + t * r);
                }
            }
        }

        return (false, Vec2::default());
    }
}

// rust unit test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_intersection() {
        let ray = Ray {
            origin: Vec2::new(0., 0.),
            dir: Vec2::new(1., 0.),
        };
        let seg = Segment {
            src: Vec2::new(1., -2.),
            dst: Vec2::new(1., 2.),
        };
        assert_eq!(ray.intersection(seg), (true, Vec2::new(1., 0.)));
    }

    #[test]
    fn test_ray_intersection2() {
        let ray = Ray {
            origin: Vec2::new(0., 0.),
            dir: Vec2::new(1., 1.),
        };
        let seg = Segment {
            src: Vec2::new(1., -2.),
            dst: Vec2::new(1., 2.),
        };
        assert_eq!(ray.intersection(seg), (true, Vec2::new(1., 1.)));
    }

    #[test]
    fn test_ray_intersection3() {
        // shouldn't intersect
        let ray = Ray {
            origin: Vec2::new(0., 0.),
            dir: Vec2::new(-1., 0.),
        };
        let seg = Segment {
            src: Vec2::new(1., -2.),
            dst: Vec2::new(1., 2.),
        };
        assert_eq!(ray.intersection(seg), (false, Vec2::new(0., 0.)));
    }
}
