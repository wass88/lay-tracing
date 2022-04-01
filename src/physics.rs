use super::math_util::PI;
use super::math_util::V3;

pub struct Ball {
    pub pos: V3,
    pub radius: f64,
    pub speed: V3,
}
pub struct World {
    pub balls: Vec<Ball>,
    pub bump: f64,
    pub gravity: f64,
}
impl World {
    pub fn tick(&mut self) {
        {
            let balls = &mut self.balls;
            for ball in balls {
                ball.pos = ball.pos + ball.speed;
                ball.speed.1 -= self.gravity;
                if ball.pos.1 < ball.radius {
                    ball.speed.1 = -self.bump * ball.speed.1;
                    ball.pos.1 = ball.radius;
                }
            }
        }
        let balls = &mut self.balls;
        for a in 0..balls.len() {
            for b in (a + 1)..balls.len() {
                let ab = balls[a].pos - balls[b].pos;
                let ra = balls[a].radius;
                let rb = balls[b].radius;
                let gap = ab.len() - ra - rb;
                if gap < 0. {
                    let ab_norm = ab.norm();
                    let pull = ab_norm * gap * 1.000001;

                    balls[a].pos = balls[a].pos - pull * 0.5;
                    balls[b].pos = balls[b].pos + pull * 0.5;

                    let va = balls[a].speed.dot(ab_norm);
                    let vb = balls[b].speed.dot(ab_norm);
                    let (wa, wb) = collision(va, vb, ra * ra * ra, rb * rb * rb);

                    balls[a].speed = balls[a].speed - ab_norm * (va - wa);
                    balls[b].speed = balls[b].speed - ab_norm * (vb - wb);
                }
            }
        }
    }
}

fn collision(v1: f64, v2: f64, m1: f64, m2: f64) -> (f64, f64) {
    let w1 = ((m2 + m2) * v2 + (m1 - m2) * v1) / (m1 + m2);
    let w2 = ((m1 + m1) * v1 + (m2 - m1) * v2) / (m1 + m2);
    (w1, w2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision() {
        let mut world = World {
            balls: vec![
                Ball { pos: V3(0., 10., 0.), radius: 4., speed: V3(0., 2., 0.) },
                Ball { pos: V3(0., 25., 0.), radius: 3., speed: V3(0., -0.7, 0.) },
            ],
            bump: 0.5,
            gravity: 0.05,
        };
        for _ in 0..20 {
            let w = 50;
            let mut str = vec!["_"; w];
            let conv = |x: f64| -> usize { (x as usize).min(w - 1).max(0) };
            str[conv(world.balls[0].pos.1)] = "1";
            str[conv(world.balls[1].pos.1)] = "2";
            println! {"{}", str.join("")}
            world.tick();
        }
    }
}
