extern crate image;

mod v3;
use v3::V3;
type Point = V3;
type Color = V3;

pub struct World {
    //objects: Vec<Geom>,
    camera: Camera,
}
struct Camera {
    pos: Point,
    center: Point,
    roll: V3,
    width: f64,
    height: f64,
}
pub struct RenderOption {
    pub campus_width: u32,
    pub campus_height: u32,
}
impl World {
    pub fn new() -> World {
        /*let objects = vec![Geom::Plain {
            origin: V3(0., 0., 0.),
            x: V3(1., 0., 0.),
            y: V3(0., 1., 0.),
            color: V3(0., 1., 0.),
        }];
        */
        let camera = Camera {
            pos: V3(0., 0., 0.),
            center: V3(0., 0., -1.),
            roll: V3(1., 0., 0.),
            width: 2. * 16. / 9.,
            height: 2.,
        };
        World { /*  objects, */ camera }
    }
    pub fn render(&self, option: RenderOption) -> image::RgbImage {
        let mut buf = image::RgbImage::new(option.campus_width, option.campus_height);

        for x in 0..option.campus_width {
            for y in 0..option.campus_height {
                let V3(r, g, b) = self.pixel(
                    x as f64 / option.campus_width as f64,
                    y as f64 / option.campus_height as f64,
                    option.campus_width as f64 / option.campus_height as f64,
                );
                let r = (r * 255.) as u8;
                let g = (g * 255.) as u8;
                let b = (b * 255.) as u8;
                buf.put_pixel(x, y, image::Rgb([r, g, b]))
            }
        }
        return buf;
    }
    fn pixel(&self, x: f64, y: f64, aspect: f64) -> V3 {
        let camera = &self.camera;
        let ray_pos = camera.pos;
        let roll_y = (camera.center - camera.pos).cross(camera.roll);
        let ray_to = camera.center
            + camera.roll * camera.width * 0.5 * (x - 0.5)
            + roll_y * camera.height * 0.5 * (y - 0.5);
        let ray_way = (ray_to - ray_pos).norm();
        let ray = Ray { pos: ray_pos, way: ray_way };

        let sphere = Sphere { pos: V3(0., 0., -1.), radius: 0.1 };
        let hit = sphere.hit(ray);
        if hit >= 0. {
            let sphere_color = V3(1., 0., 0.);
            let hit_pos = ray.at(hit);
            let sphere_color = V3(hit_pos.0 + 0.5, hit_pos.1 + 0.5, hit_pos.2 + 0.5);
            return sphere_color;
        } else {
            let t = 0.5 * (ray.way.1 + 1.);
            let back = V3(1., 1., 1.) * (1.0 - t) + V3(0.5, 0.7, 1.);
            return back;
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    pos: Point,
    way: V3,
}
impl Ray {
    fn at(self, k: f64) -> Point {
        self.pos + self.way * k
    }
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    pos: Point,
    radius: f64,
}
impl Sphere {
    fn hit(self, ray: Ray) -> f64 {
        let rw = ray.pos - self.pos;
        let ra = ray.way.sq_len();
        let rb = ray.way.dot(rw);
        let rc = rw.sq_len() - self.radius;
        let det = rb * rb - ra * rc;
        if det < 0. {
            -1.
        } else {
            (-rb - det.sqrt()) / ra
        }
    }
}
