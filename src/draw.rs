use crate::fatrix::{Float, Point, Space};

const RESOLUTION: usize = 20;

///Adds a circle to a given fatrix
///circle defined by its center point (cx, cy, cz) and a radius, r
pub fn add_circle(cx: Float, cy: Float, cz: Float, r: Float, edges: &mut Space) {
    for t in 0..RESOLUTION {
        //RESOLUTION should be reasonable enough that these type casts are fine
        let t0 = (t as Float) / (RESOLUTION as Float) * 2.0 * std::f32::consts::PI;
        let t1 = ((t + 1) as Float) / (RESOLUTION as Float) * 2.0 * std::f32::consts::PI;
        let x0 = r * t0.cos() + cx;
        let y0 = r * t0.sin() + cy;
        let x1 = r * t1.cos() + cx;
        let y1 = r * t1.sin() + cy;
        edges.add_line((x0, y0, cz), (x1, y1, cz));
    }
}

///Adds a hermite curve defined by a start and end point and slopes coming out of or into those
///points
pub fn add_hermite(
    x0: Float,
    y0: Float,
    x1: Float,
    y1: Float,
    rx0: Float,
    ry0: Float,
    rx1: Float,
    ry1: Float,
    edges: &mut Space,
) {
    let ax = 2.0 * x0 - 2.0 * x1 + rx0 + rx1;
    let bx = -3.0 * x0 + 3.0 * x1 - 2.0 * rx0 - rx1;
    let cx = rx0;
    let dx = x0;
    let ay = 2.0 * y0 - 2.0 * y1 + ry0 + ry1;
    let by = -3.0 * y0 + 3.0 * y1 - 2.0 * ry0 - ry1;
    let cy = ry0;
    let dy = y0;
    let fx = |x| ax * x * x * x + bx * x * x + cx * x + dx;
    let fy = |y| ay * y * y * y + by * y * y + cy * y + dy;
    for t in 0..RESOLUTION {
        let t0 = (t as Float) / (RESOLUTION as Float);
        let t1 = ((t + 1) as Float) / (RESOLUTION as Float);
        let px0 = fx(t0);
        let py0 = fy(t0);
        let px1 = fx(t1);
        let py1 = fy(t1);
        edges.add_line((px0, py0, 0.0), (px1, py1, 0.0));
    }
}

///adds bezier curve to fatrix with (x0, y0) and (x3, y3) as start and end points and the other two
///points control points
pub fn add_bezier(
    x0: Float,
    y0: Float,
    x1: Float,
    y1: Float,
    x2: Float,
    y2: Float,
    x3: Float,
    y3: Float,
    edges: &mut Space,
) {
    let ax = -x0 + 3.0 * x1 - 3.0 * x2 + x3;
    let bx = 3.0 * x0 - 6.0 * x1 + 3.0 * x2;
    let cx = -3.0 * x0 + 3.0 * x1;
    let dx = x0;
    let ay = -y0 + 3.0 * y1 - 3.0 * y2 + y3;
    let by = 3.0 * y0 - 6.0 * y1 + 3.0 * y2;
    let cy = -3.0 * y0 + 3.0 * y1;
    let dy = y0;
    let fx = |x| ax * x * x * x + bx * x * x + cx * x + dx;
    let fy = |y| ay * y * y * y + by * y * y + cy * y + dy;
    for t in 0..RESOLUTION {
        //cast should be fine if resolution is not stupid
        let t0 = (t as Float) / (RESOLUTION as Float);
        let t1 = ((t + 1) as Float) / (RESOLUTION as Float);
        let px0 = fx(t0);
        let py0 = fy(t0);
        let px1 = fx(t1);
        let py1 = fy(t1);
        edges.add_line((px0, py0, 0.0), (px1, py1, 0.0));
    }
}

///adds a box to the given fatrix given the front top left corner x, y, z and a width, height, and
///depth
pub fn add_box(x: Float, y: Float, z: Float, w: Float, h: Float, d: Float, edges: &mut Space) {
    //front face
    edges.add_tri((x, y, z), (x, y - h, z), (x + w, y - h, z));
    edges.add_tri((x, y, z), (x + w, y - h, z), (x + w, y, z));
    //right face
    edges.add_tri((x + w, y, z), (x + w, y - h, z), (x + w, y - h, z - d));
    edges.add_tri((x + w, y, z), (x + w, y - h, z - d), (x + w, y, z- d));

    //back face
    edges.add_tri((x, y, z - d), (x + w, y - h, z - d), (x, y - h, z - d));
    edges.add_tri((x + w, y, z - d), (x + w, y - h, z - d), (x, y , z - d));

    //left face
    edges.add_tri((x, y, z), (x, y - h, z - d), (x, y - h, z));
    edges.add_tri((x, y, z), (x, y, z - d), (x, y - h, z - d));

    //top face
    edges.add_tri((x + w, y, z - d), (x, y, z), (x + w, y, z));
    edges.add_tri((x + w, y, z - d), (x, y, z - d), (x, y, z));

    //bot face
    edges.add_tri((x + w, y - h, z), (x, y - h, z), (x + w, y - h, z - d));
    edges.add_tri((x, y - h, z), (x, y - h, z - d), (x + w, y - h, z - d));
}

///returns a vector of the points on the sphere
fn sphere_points(x: Float, y: Float, z: Float, r: Float) -> Vec<Point> {
    //this conversion should be fine as long as usize isn't stupid as well
    let mut out = Vec::with_capacity(RESOLUTION * RESOLUTION);
    for p in 0..RESOLUTION {
        for t in 0..=RESOLUTION-1 {
            //cast should be fine as resolution is not stupid
            let phi = (p as Float) / ((RESOLUTION - 1) as Float) * 2.0 * std::f32::consts::PI;
            let theta = (t as Float) / ((RESOLUTION - 1) as Float) * std::f32::consts::PI;
            let px = r * theta.cos() + x;
            let py = r * theta.sin() * phi.cos() + y;
            let pz = r * theta.sin() * phi.sin() + z;
            out.push((px, py, pz));
        }
    }
    out
}

///adds a sphere to a fatrix given a center (x, y, z) and a radius r
pub fn add_sphere(x: Float, y: Float, z: Float, r: Float, edges: &mut Space) {
    let p = sphere_points(x, y, z, r);
    let n = p.len();
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION-1 {
            let o1 = (j + i * RESOLUTION) % n;
            let o2 = (j + 1 + i * RESOLUTION) % n;
            let o3 = (j + RESOLUTION + 1 + i * RESOLUTION) % n;
            let o4 = (j + RESOLUTION + i * RESOLUTION) % n;

            if j == 0 {
                edges.add_tri(p[o1], p[o2], p[o3]);
            } else if j + 1 == RESOLUTION-1 {
                edges.add_tri(p[o1], p[o3], p[o4]);
            } else {
                edges.add_tri(p[o1], p[o2], p[o3]);
                edges.add_tri(p[o1], p[o3], p[o4]);
            }

        }
    }
}

///adds a torus to a fatrix given the center point (x, y, z) the radius of a cross section, r1, and
///the radius from the center point to the outer edge, r2
fn torus_points(x: Float, y: Float, z: Float, r1: Float, r2: Float) -> Vec<Point> {
    let mut out = Vec::with_capacity(RESOLUTION * RESOLUTION);
    for p in 0..RESOLUTION {
        for t in 0..RESOLUTION {
            let phi = (p as Float) / (RESOLUTION as Float) * 2.0 * std::f32::consts::PI;
            let theta = (t as Float) / (RESOLUTION as Float) * 2.0 * std::f32::consts::PI;
            let px = phi.cos() * (r1 * theta.cos() + r2) + x;
            let py = r1 * theta.sin() + y;
            let pz = phi.sin() * (r1 * theta.cos() + r2) + z;

            out.push((px, py, pz));
        }
    }
    out
}

pub fn add_torus(x: Float, y: Float, z: Float, r1: Float, r2: Float, edges: &mut Space) {
    let p = torus_points(x, y, z, r1, r2);
    let n = p.len();
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let o1 = (j + i * RESOLUTION) % n;
            let o2 = ((j + 1) % RESOLUTION + i * RESOLUTION) % n;
            let o3 = ((j + 1) % RESOLUTION + RESOLUTION + i * RESOLUTION) % n;
            let o4 = (j + RESOLUTION + i * RESOLUTION) % n;
            edges.add_tri(p[o1], p[o2], p[o3]);
            edges.add_tri(p[o1], p[o3], p[o4]);
        }
    }
}
