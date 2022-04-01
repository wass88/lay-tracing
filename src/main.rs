use lay_tracing::*;
use std::{str::MatchIndices, sync::Arc};

#[tokio::main]
async fn main() {
    let look_from = V3(1.1, 0.3, 0.2);
    let look_at = V3(0., 0., -0.5);
    let up = V3(0., -1., 0.);
    let focus_dist = (look_from - look_at).len();
    let aperture = 0.02;
    let camera = Camera::new(look_from, look_at, up, 0.3 * PI, 16. / 9., aperture, focus_dist);

    let material_ground = Arc::new(Lambertian { color: V3(0.8, 0.8, 0.1) });
    let material_left = Arc::new(Glass { ir: 2.0 });
    let material_center = Arc::new(Metal { color: V3(0.8, 0.8, 0.8), fuzz: 0.01 });
    let material_right = Arc::new(Lambertian { color: V3(0.8, 0.6, 0.2) });

    let geom_ground = Arc::new(Sphere {
        pos: V3(0., -100. - 0.4, -1.),
        radius: -100.,
        material: material_ground,
    });
    let mut spheres: Vec<Arc<Sphere>> = vec![];
    spheres.push(geom_ground);

    let mut ph = physics::World { balls: vec![], bump: 0.5, gravity: 0.01 };
    let mut mats: Vec<Arc<dyn Material>> = vec![];
    ph.balls.push(physics::Ball { pos: V3(0.8, 0.4, -1.), radius: 0.4, speed: V3(0., 0., 0.) });
    mats.push(material_left);
    ph.balls.push(physics::Ball { pos: V3(0., 0.4, -1.), radius: 0.4, speed: V3(0., 0., 0.) });
    mats.push(material_center);
    ph.balls.push(physics::Ball { pos: V3(-0.8, 0.4, -1.), radius: 0.4, speed: V3(0., 0., 0.) });
    mats.push(material_right);

    fn ball_sphere(ball: &physics::Ball, material: Arc<dyn Material>) -> Arc<Sphere> {
        let V3(x, y, z) = ball.pos;
        let pos = V3(x, y - 0.4, z);
        Arc::new(Sphere { pos, radius: ball.radius, material: material })
    }

    let balls = 100;
    for i in 3..balls {
        let mat = rand();
        let glass = Arc::new(Glass { ir: rand() + 1. });
        let metal = Arc::new(Metal { color: rand_v3() * rand_v3(), fuzz: 0.1 });
        let c = V3(rand().sqrt(), rand().sqrt(), rand().sqrt());
        let lamber = Arc::new(Lambertian { color: c });
        let material: Arc<dyn Material> = if mat > 0.6 {
            glass
        } else if mat > 0.3 {
            metal
        } else {
            lamber
        };
        ph.balls.push(physics::Ball {
            pos: V3(rand() * -6. + 3., 0.5, -rand() * 1.8 + 1.),
            radius: 0.05,
            speed: rand_v3() * 0.1,
        });
        mats.push(material);
    }
    for i in 0..balls {
        let sphere = ball_sphere(&ph.balls[i], mats[i].clone());
        spheres.push(sphere)
    }
    let good_option = Arc::new(lay_tracing::RenderOption {
        campus_width: (1024. * (16. / 9.)) as u32,
        campus_height: 1024,
        samples: 10,
        depth: 10,
    });
    let bad_option = Arc::new(lay_tracing::RenderOption {
        campus_width: (256. * (16. / 9.)) as u32 + 1,
        campus_height: 256,
        samples: 3,
        depth: 3,
    });
    let max_frame = 60;
    for i in 0..max_frame {
        eprintln!("rendering... {:04}/{}", i, max_frame);
        let mut world = World::new(camera.clone());
        for s in 0..balls + 1 {
            world.objects.geoms.push(spheres[s].clone())
        }

        let img = World::render(Arc::new(world), good_option.clone()).await;
        img.save(format!("movie/movie{:04}.png", i)).unwrap();
        ph.tick();
        for b in 0..balls {
            spheres[b + 1] = ball_sphere(&ph.balls[b], mats[b].clone())
        }
    }
}
