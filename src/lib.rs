extern crate image;
extern crate rand;

mod math_util;
use math_util::*;

type Point = V3;
type Color = V3;
use std::rc::Rc;

pub struct World {
    objects: GeomList,
    camera: Camera,
}
struct Camera {
    pos: Point,
    lower_left: Point,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f64,
}
impl Camera {
    fn new(
        look_from: Point,
        look_at: Point,
        up: V3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let h = (vfov * 0.5).tan();
        let height = 2. * h;
        let width = aspect * height;

        let w = (look_from - look_at).norm();
        let u = up.norm().cross(w);
        let v = w.cross(u);

        let horizontal = u * width * focus_dist;
        let vertical = v * height * focus_dist;
        let lower_left = look_from - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;

        Camera {
            pos: look_from,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            w,
            lens_radius: aperture * 0.5,
        }
    }
    fn get_ray(&self, x: f64, y: f64) -> Ray {
        let lens_disk = rand_disk() * self.lens_radius;
        let offset = self.u * lens_disk.0 + self.v * lens_disk.1;
        Ray {
            pos: self.pos + offset,
            way: (self.lower_left + self.horizontal * x + self.vertical * y - self.pos - offset)
                .norm(),
        }
    }
}
pub struct RenderOption {
    pub campus_width: u32,
    pub campus_height: u32,
    pub depth: usize,
    pub samples: usize,
}
impl World {
    pub fn new() -> World {
        let material_ground = Rc::new(Lambertian { color: V3(0.8, 0.8, 0.1) });
        let material_left = Rc::new(Glass { ir: 2.0 });
        let material_center = Rc::new(Glass { ir: 10.0 });
        let material_right = Rc::new(Lambertian { color: V3(0.8, 0.6, 0.2) });

        let geom_ground = Box::new(Sphere {
            pos: V3(0., -10. - 0.4, -1.),
            radius: -10.,
            material: material_ground,
        });
        let geom_left =
            Box::new(Sphere { pos: V3(0.8, 0., -1.), radius: 0.4, material: material_left });
        let geom_center =
            Box::new(Sphere { pos: V3(0., 0., -1.), radius: 0.4, material: material_center });
        let geom_right =
            Box::new(Sphere { pos: V3(-0.8, 0., -1.), radius: 0.4, material: material_right });
        let objects = GeomList { geoms: vec![geom_left, geom_center, geom_right, geom_ground] };
        /*vec![Geom::Plain {
            origin: V3(0., 0., 0.),
            x: V3(1., 0., 0.),
            y: V3(0., 1., 0.),
            color: V3(0., 1., 0.),
        }]*/
        let look_from = V3(1.2, 0.9, 0.7);
        let look_at = V3(0., 0., -1.);
        let up = V3(0., -1., 0.);
        let focus_dist = (look_from - look_at).len();
        let aperture = 0.8;
        let camera = Camera::new(look_from, look_at, up, 0.3 * PI, 16. / 9., aperture, focus_dist);
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
                    let color = self.pixel(sx, sy, option.depth);
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
    fn pixel(&self, x: f64, y: f64, depth: usize) -> Color {
        let camera = &self.camera;
        self.ray_color(camera.get_ray(x, y), depth)
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

#[derive(Debug, Clone, Copy)]
struct Lambertian {
    color: Color,
}
impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let mut way = hit.normal + rand_hemisphere(hit.normal);
        if way.near_zero() {
            way = hit.normal;
        }
        let ray = Ray { pos: hit.pos, way: way.norm() };
        (self.color, Some(ray))
    }
}

#[derive(Debug, Clone, Copy)]
struct Metal {
    color: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let reflect = ray.way.reflect(hit.normal);
        let way = reflect + rand_hemisphere(reflect) * self.fuzz;
        let ray = Ray { pos: hit.pos, way: way.norm() };
        (self.color, Some(ray))
    }
}

#[derive(Debug, Clone, Copy)]
struct Glass {
    ir: f64,
}

impl Material for Glass {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> (Color, Option<Ray>) {
        let ref_ratio = if hit.front_face { 1. / self.ir } else { self.ir };
        let cos = -ray.way.dot(hit.normal);
        let sin = (1. - cos * cos).sqrt();
        let cannot_refracted = ref_ratio * sin > 1.;

        let way = if cannot_refracted {
            ray.way.reflect(hit.normal)
        } else {
            ray.way.refract(hit.normal, ref_ratio)
        };
        let ray = Ray { pos: hit.pos, way: way.norm() };
        (V3(1., 1., 1.), Some(ray))
    }
}
