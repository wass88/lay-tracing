extern crate image;
extern crate rand;

mod v3;
use v3::V3;
type Point = V3;
type Color = V3;
use std::f64::consts::PI;
use std::rc::Rc;

fn rand_in(min: f64, max: f64) -> f64 {
    use crate::rand::distributions::Distribution;
    rand::distributions::Uniform::from(min..max).sample(&mut rand::thread_rng())
}
fn rand() -> f64 {
    rand_in(0., 1.)
}
fn rand_unit_v3() -> V3 {
    let phi = rand_in(0., 2. * PI);
    let ctheta = rand_in(-1., 1.);
    let theta = ctheta.acos();
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    V3(x, y, z)
}
fn rand_hemisphere(normal: V3) -> V3 {
    let v = rand_unit_v3();
    if normal.dot(v) > 0. {
        v
    } else {
        -v
    }
}

pub struct World {
    objects: GeomList,
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
    pub samples: usize,
}
impl World {
    pub fn new() -> World {
        let material = Rc::new(Lambertian { albedo: V3(0.2, 0.8, 0.1) });
        let objects = GeomList {
            geoms: vec![
                Box::new(Sphere { pos: V3(0., 0., -1.), radius: 0.4, material: material.clone() }),
                Box::new(Sphere {
                    pos: V3(0., -10. - 0.4, -1.),
                    radius: -10.,
                    material: material.clone(),
                }),
            ],
        };
        /*vec![Geom::Plain {
            origin: V3(0., 0., 0.),
            x: V3(1., 0., 0.),
            y: V3(0., 1., 0.),
            color: V3(0., 1., 0.),
        }]*/
        let camera = Camera {
            pos: V3(0., 0., 0.),
            center: V3(0., 0., -1.),
            roll: V3(1., 0., 0.),
            width: 2. * 16. / 9.,
            height: 2.,
        };
        World { objects, camera }
    }
    pub fn render(&self, option: RenderOption) -> image::RgbImage {
        let mut buf = image::RgbImage::new(option.campus_width, option.campus_height);

        for x in 0..option.campus_width {
            for y in 0..option.campus_height {
                let mut total_color = V3(0., 0., 0.);
                for _ in 0..option.samples {
                    let sx = (x as f64 + rand()) / option.campus_width as f64;
                    let sy = (y as f64 + rand()) / option.campus_height as f64;
                    let color = self.pixel(
                        sx,
                        sy,
                        option.campus_width as f64 / option.campus_height as f64,
                    );
                    total_color = total_color + color;
                }
                let V3(r, g, b) = total_color / (option.samples as f64);
                let r = (r.sqrt() * 255.) as u8;
                let g = (g.sqrt() * 255.) as u8;
                let b = (b.sqrt() * 255.) as u8;
                buf.put_pixel(x, y, image::Rgb([r, g, b]))
            }
        }
        return buf;
    }
    fn pixel(&self, x: f64, y: f64, aspect: f64) -> Color {
        let camera = &self.camera;
        let ray_pos = camera.pos;
        let roll_y = (camera.center - camera.pos).cross(camera.roll);
        let ray_to = camera.center
            + camera.roll * camera.width * 0.5 * (x - 0.5)
            + roll_y * camera.height * 0.5 * (y - 0.5);
        let ray_way = (ray_to - ray_pos).norm();
        let ray = Ray { pos: ray_pos, way: ray_way };
        self.ray_color(ray, 3)
    }
    fn ray_color(&self, ray: Ray, depth: usize) -> Color {
        if depth <= 0 {
            return V3(0., 0., 0.);
        }
        let hit = self.objects.hit(ray, 0.0001, 10.);
        if let Some(hit) = hit {
            let (color, scatter) = hit.material.scatter(ray, &hit);
            if let Some(scatter) = scatter {
                color * self.ray_color(scatter, depth - 1)
            } else {
                V3(0., 0., 0.)
            }
        } else {
            let t = 0.5 * (ray.way.1 + 1.);
            let back = V3(1., 1., 1.) * (1.0 - t) + V3(0.5, 0.7, 1.) * t;
            back
        }
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

#[derive(Debug, Clone)]
struct HitRecord {
    pos: Point,
    normal: V3,
    distance: f64,
    front_face: bool,
    material: Rc<dyn Material>,
}
impl HitRecord {
    fn new_normal(
        ray: Ray,
        pos: Point,
        normal: V3,
        distance: f64,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        let front_face = ray.way.dot(normal) < 0.;
        let normal = if front_face { normal } else { -normal };
        HitRecord { pos, normal, distance, front_face, material }
    }
}

trait Geom: std::fmt::Debug {
    fn hit(&self, ray: Ray, d_min: f64, d_max: f64) -> Option<HitRecord>;
}

#[derive(Debug)]
struct GeomList {
    geoms: Vec<Box<dyn Geom>>,
}
impl Geom for GeomList {
    fn hit(&self, ray: Ray, d_min: f64, d_max: f64) -> Option<HitRecord> {
        let mut nearest: Option<HitRecord> = None;
        for geom in &self.geoms {
            if let Some(hit) = geom.hit(ray, d_min, d_max) {
                if nearest.is_none() || hit.distance < nearest.as_ref().unwrap().distance {
                    nearest = Some(hit)
                }
            }
        }
        return nearest;
    }
}
impl GeomList {
    fn add(&mut self, geom: Box<dyn Geom>) {
        self.geoms.push(geom)
    }
    fn clear(&mut self) {
        self.geoms.clear()
    }
}

#[derive(Debug, Clone)]
struct Sphere {
    pos: Point,
    radius: f64,
    material: Rc<dyn Material>,
}
impl Geom for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rw = ray.pos - self.pos;
        let ra = ray.way.sq_len();
        let rb = ray.way.dot(rw);
        let rc = rw.sq_len() - self.radius * self.radius;
        let det = rb * rb - ra * rc;
        if det < 0. {
            return None;
        }
        let d0 = (-rb - det.sqrt()) / ra;
        let d1 = (-rb + det.sqrt()) / ra;
        let d = if t_min < d0 && d0 < t_max {
            d0
        } else if t_min < d1 && d1 < t_max {
            d1
        } else {
            return None;
        };
        let distance = d;
        let pos = ray.at(distance);
        let normal = (pos - self.pos).norm();
        Some(HitRecord::new_normal(ray, pos, normal, distance, self.material.clone()))
    }
}

trait Material: std::fmt::Debug {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Color, Option<Ray>);
}

#[derive(Debug, Clone)]
struct Lambertian {
    albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let mut way = hit.normal + rand_hemisphere(hit.normal);
        if way.near_zero() {
            way = hit.normal;
        }
        let ray = Ray { pos: hit.pos, way: way.norm() };
        (self.albedo, Some(ray))
    }
}
struct Metal {}
