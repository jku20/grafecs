use crate::fatrix::{Float, Fatrix, };

const RESOLUTION: f32  = 0.01;

pub fn add_circle(cx: Float, cy: Float, cz: Float, r: Float, edges: &mut Fatrix) {
    let mut t = RESOLUTION;
    while t <= 1.0 {
        let t0 = (t - RESOLUTION) * 2.0 * std::f32::consts::PI;
        let t1 = t * 2.0 * std::f32::consts::PI;
        let x0 = r * t0.cos() + cx;
        let y0 = r * t0.sin() + cy;
        let x1 = r * t1.cos() + cx;
        let y1 = r * t1.sin() + cy;
        edges.add_edge((x0, y0, cz), (x1, y1, cz));
        t += RESOLUTION;
    }
}

pub fn add_hermite(x0: Float, y0: Float, x1: Float, y1: Float, rx0: Float, ry0: Float, rx1: Float, ry1: Float, edges: &mut Fatrix) {
    let mut t = RESOLUTION;
    let ax = 2.0 * x0 - 2.0 * x1 + rx0 + rx1;
    let bx = -3.0 * x0 + 3.0 * x1 - 2.0 * rx0 - rx1;
    let cx = rx0;
    let dx = x0;
    let ay = 2.0 * y0 - 2.0 * y1 + ry0 + ry1;
    let by = -3.0 * y0 + 3.0 * y1 - 2.0 * ry0 - ry1;
    let cy = ry0;
    let dy = y0;
    let fx = |x| ax * x * x * x + bx * x * x +  cx * x + dx;
    let fy = |y| ay * y * y * y + by * y * y +  cy * y + dy;
    while t <= 1.0 {
        let t0 = t - RESOLUTION;
        let t1 = t;
        let px0 = fx(t0);
        let py0 = fy(t0);
        let px1 = fx(t1);
        let py1 = fy(t1);
        edges.add_edge((px0, py0, 0.0), (px1, py1, 0.0));
        t += RESOLUTION;
    }
}

pub fn add_bezier(x0: Float, y0: Float, x1: Float, y1: Float, x2: Float, y2: Float, x3: Float, y3: Float, edges: &mut Fatrix) {
    let mut t = RESOLUTION;
    let ax = -x0 + 3.0 * x1 - 3.0 * x2 + x3;
    let bx = 3.0 * x0 - 6.0 * x1 + 3.0 * x2;
    let cx = -3.0 * x0 + 3.0 * x1;
    let dx = x0;
    let ay = -y0 + 3.0 * y1 - 3.0 * y2 + y3;
    let by = 3.0 * y0 - 6.0 * y1 + 3.0 * y2;
    let cy = -3.0 * y0 + 3.0 * y1;
    let dy = y0;
    let fx = |x| ax * x * x * x + bx * x * x +  cx * x + dx;
    let fy = |y| ay * y * y * y + by * y * y +  cy * y + dy;
    while t <= 1.0 {
        let t0 = t - RESOLUTION;
        let t1 = t;
        let px0 = fx(t0);
        let py0 = fy(t0);
        let px1 = fx(t1);
        let py1 = fy(t1);
        edges.add_edge((px0, py0, 0.0), (px1, py1, 0.0));
        t += RESOLUTION;
    }
}
