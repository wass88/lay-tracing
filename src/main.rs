use lay_tracing::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let look_from = V3(1.1, 0.3, 0.2);
    let look_at = V3(0., 0., -0.5);
    let up = V3(0., -1., 0.);
    let focus_dist = (look_from - look_at).len();
    let aperture = 0.02;
    let camera = Camera::new(look_from, look_at, up, 0.3 * PI, 16. / 9., aperture, focus_dist);

    let mut world = World::new(camera);

    let material_ground = Arc::new(Lambertian { color: V3(0.8, 0.8, 0.1) });
    let material_left = Arc::new(Glass { ir: 2.0 });
    let material_center = Arc::new(Metal { color: V3(0.8, 0.8, 0.8), fuzz: 0.01 });
    let material_right = Arc::new(Lambertian { color: V3(0.8, 0.6, 0.2) });

    let geom_ground = Box::new(Sphere {
        pos: V3(0., -100. - 0.4, -1.),
        radius: -100.,
        material: material_ground,
    });
    let geom_left =
        Box::new(Sphere { pos: V3(0.8, 0., -1.), radius: 0.4, material: material_left });
    let geom_center =
        Box::new(Sphere { pos: V3(0., 0., -1.), radius: 0.4, material: material_center });
    let geom_right =
        Box::new(Sphere { pos: V3(-0.8, 0., -1.), radius: 0.4, material: material_right });

    world.add(geom_ground);
    world.add(geom_left);
    world.add(geom_center);
    world.add(geom_right);

    let childs = 100;
    for _ in 0..childs {
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
        let sphere = Box::new(Sphere {
            pos: V3(rand() * -6. + 3., -0.4 + 0.5 * 0.05, -rand() * 1.8 + 1.),
            radius: 0.05,
            material,
        });
        world.add(sphere)
    }

    let option = lay_tracing::RenderOption {
        campus_width: (1024. * (16. / 9.)) as u32,
        campus_height: 1024,
        samples: 10,
        depth: 10,
    };
    World::render(Arc::new(world), Arc::new(option)).await.save("test.png").unwrap();
}
