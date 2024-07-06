use crate::vec::Vec3;

fn distance(pointa: Vec3<f32>, pointb: Vec3<f32>) -> f32 {
    ((pointb.x - pointa.x).powf(2.0)
        + ((pointb.y - pointa.y).powf(2.0) + (pointb.z - pointa.z).powf(2.0)))
    .sqrt()
}

fn sphere(position: Vec3<f32>, center: Vec3<f32>, radius: f32) -> f32 {
    distance(position, center) - radius
}

pub fn scene(position: Vec3<f32>, time: f32) -> f32 {
    op_smooth_union(
        sphere(position, Vec3::new(-0.2, 0., 0.), 0.5),
        sphere(position, Vec3::new(0.7, 0., time.sin()), 0.5), 
        0.5)
}

fn op_smooth_union(distance1:f32, distance2:f32, smooth:f32 )->f32
{
    let h = ( 0.5 + 0.5*(distance2-distance1)/smooth).clamp(0.0, 1.0);
    return mix( distance2, distance1, h ) - smooth*h*(1.0-h);
}

fn mix(x:f32, y:f32, a:f32)->f32{
    x*(1.-a) + a*y
}

pub fn normalize(vector: Vec3<f32>) -> Vec3<f32> {
    let length = (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0)).sqrt();
    Vec3 {
        x: (vector.x / length),
        y: (vector.y / length),
        z: (vector.z / length),
    }
}


pub fn compute_normal(position: Vec3<f32>, time:f32) -> Vec3<f32> {
    let epsilon = 0.001;
    normalize(Vec3 {
        x: scene(
            position
                + Vec3 {
                    x: epsilon,
                    y: 0.,
                    z: 0.,
                },
         time) - scene(position, time),
        y: scene(
            position
                + Vec3 {
                    x: 0.,
                    y: epsilon,
                    z: 0.,
                },
         time) - scene(position, time),
        z: scene(
            position
                + Vec3 {
                    x: 0.,
                    y: 0.,
                    z: epsilon,
                },
        time) - scene(position, time),
    })
}