use crate::util::{Shape2D, Vec2};

pub fn intersection<'a>(a: &Box<dyn Shape2D + Send + Sync + 'a>, a_pos: Vec2, b: &Box<dyn Shape2D + Send + Sync + 'a>, b_pos: Vec2) -> Option<Vec2> {
    let a = a.shifted(&a_pos);
    let b = b.shifted(&b_pos);

    let mut overlap = f32::MAX;
    let mut smallest = Vec2::zeros();
    let a_axes = a.get_axes();
    let b_axes = b.get_axes();

    fn separated(a: (f32, f32), b: (f32, f32)) -> bool {
        a.1 < b.0 || b.1 < a.0
    }

    fn get_overlap(a: (f32, f32), b: (f32, f32)) -> f32 {
        a.1.min(b.1) - a.0.max(b.0)
    }

    fn contains(a: (f32, f32), b: (f32, f32)) -> bool {
        a.0 < b.0 && a.1 > b.1
    }

    for axis in a_axes {
        let p1 = a.project(&axis);
        let p2 = b.project(&axis);

        if separated(p1, p2) {
            return None;
        } else {
            let mut o = get_overlap(p1, p2);

            if contains(p1, p2) || contains(p2, p1) {
                let mins = (p1.0 - p2.0).abs();
                let maxs = (p1.1 - p2.1).abs();

                if mins < maxs {
                    o += mins;
                } else {
                    o += maxs;
                }
            }

            if o < overlap {
                overlap = o;
                smallest = axis;
            }
        }
    }

    for axis in b_axes {
        let p1 = a.project(&axis);
        let p2 = b.project(&axis);

        if separated(p1, p2) {
            return None;
        } else {
            let mut o = get_overlap(p1, p2);

            if contains(p1, p2) || contains(p2, p1) {
                let mins = (p1.0 - p2.0).abs();
                let maxs = (p1.1 - p2.1).abs();

                if mins < maxs {
                    o += mins;
                } else {
                    o += maxs;
                }
            }

            if o < overlap {
                overlap = o;
                smallest = axis;
            }
        }
    }

    Some(smallest * overlap)
}