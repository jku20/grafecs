use crate::screen::{color::Color, Screen};

type Point = (i32, i32);

///Plots a point p to the screen.
///Points which are off the screen will be ignored.
///Should never panic except for really weird cases I don't understand due to the order of the checks.
pub fn plot<T: Color>(p: Point, color: T, scrn: &mut Screen<T>) {
    if p.0 >= 0 && p.1 >= 0 && (p.0 as usize) < scrn.width() && (p.1 as usize) < scrn.height() {
        scrn[[p.0 as usize, p.1 as usize]] = color;
    }
}

///Draws a line of pixels to the screen using Bresenham's line algorithm
///or a similar algorithm described [here](https://zingl.github.io/Bresenham.pdf).
///Pixels not visable on the screen (i.e. (-1, 4)) will just be ignored.
///The pixels are inclusive meaning both p1 and p2 may be drawn
pub fn draw_line<T: Color>(p1: Point, p2: Point, color: T, scrn: &mut Screen<T>) {
    //algorithm by Alois Zingl (https://zingl.github.io/Bresenham.pdf)
    //used because it is super clean
    let dx = (p2.0 - p1.0).abs();
    let dy = -(p2.1 - p1.1).abs();
    let sx = (p2.0 - p1.0).signum();
    let sy = (p2.1 - p1.1).signum();

    let mut e = dx + dy;
    let (mut x, mut y) = (p1.0, p1.1);
    loop {
        plot((x, y), color, scrn);
        if x == p2.0 && y == p2.1 {
            break;
        }
        let et = e * 2;
        if et >= dy {
            x += sx;
            e += dy;
        }
        if et <= dx {
            y += sy;
            e += dx;
        }
    }

    //more traditional Bresenham's line algorithm implementation by me
    /*
    let lp = p1.min(p2);
    let rp = p1.max(p2);
    let dx = rp.0 - lp.0;
    let dy = rp.1 - lp.1;

    if dx > dy.abs() {
        let mut d = 2 * dy - dx;
        let tdx = 2 * dx;
        let tdy = 2 * dy;

        let mut x = lp.0;
        let mut y = lp.1;
        if dy > 0 {
            while x <= rp.0 {
                plot((x, y), color, scrn);
                if d > 0 {
                    y += 1;
                    d -= tdx;
                }
                x += 1;
                d += tdy;
            }
        } else {
            while x <= rp.0 {
                plot((x, y), color, scrn);
                if d < 0 {
                    y -= 1;
                    d += tdx;
                }
                x += 1;
                d += tdy;
            }
        }
    } else {
        let mut d = dx + 2 * dy;
        let tdx = 2 * dx;
        let tdy = 2 * dy;

        let mut x = lp.0;
        let mut y = lp.1;
        if dy > 0 {
            while y <= rp.1 {
                plot((x, y), color, scrn);
                if d > 0 {
                    x += 1;
                    d -= tdy;
                }
                y += 1;
                d += tdx;
            }
        } else {
            while y > rp.1 {
                plot((x, y), color, scrn);
                if d > 0 {
                    x += 1;
                    d += tdy;
                }
                y -= 1;
                d += tdx;
            }
        }
    }
    */
}
