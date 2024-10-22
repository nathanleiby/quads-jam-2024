use macroquad::math::Vec2;

pub struct Ray {
    pub origin: Vec2,
    pub dir: Vec2,
}

pub struct Segment {
    pub src: Vec2,
    pub dst: Vec2,
}

impl Segment {
    // returns the Direction vector of the line in parametric form (Point + Direction * Time)
    fn parametric_dir(&self) -> Vec2 {
        Vec2::new(self.dst.x - self.src.x, self.dst.y - self.src.y)
    }
}

fn cross(v: Vec2, w: Vec2) -> f32 {
    v.x * w.y - v.y * w.x
}

impl Ray {
    // https://stackoverflow.com/a/565282/950683
    pub fn intersection(&self, segment: &Segment) -> Option<Vec2> {
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
                // TODO: Ignoring collision for now. Need to compute actual intersection
                return None;
            }
            (true, false) => {
                // parallel and non-intersecting
                return None;
            }
            (false, _) => {
                // ray can extend infinitely from its source (t >= 0), but segment is bounded by its endpoints (0 <= u <= 1)
                if t >= 0. && 0. <= u && u <= 1. {
                    return Some(p + t * r);
                }
            }
        }

        return None;
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
        assert_eq!(ray.intersection(&seg), Some(Vec2::new(1., 0.)));
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
        assert_eq!(ray.intersection(&seg), Some(Vec2::new(1., 1.)));
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
        assert_eq!(ray.intersection(&seg), None);
    }

    #[test]
    fn test_ray_intersection4() {
        let ray = Ray {
            origin: Vec2::new(30., 30.),
            dir: Vec2::new(1., 0.),
        };
        let seg = Segment {
            src: Vec2::new(100., 0.),
            dst: Vec2::new(100., 100.),
        };
        assert_eq!(ray.intersection(&seg), Some(Vec2::new(100., 30.)));
    }
}
