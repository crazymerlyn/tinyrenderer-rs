extern crate image;

use image::ImageBuffer;
use image::GenericImage;

macro_rules! swap {
    ($a:ident, $b: ident) => {
        let temp = $a;
        $a = $b;
        $b = temp;
    }
}

fn line<I: GenericImage>(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, image: &mut I, color: I::Pixel) {
    let mut steep = false;
    if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
        swap!(x0, y0);
        swap!(x1, y1);
        steep = true;
    }
    if x0 > x1 {
        swap!(x0, x1);
        swap!(y0, y1);
    }
    let dx = x1 as i32 - x0 as i32;
    let dy = y1 as i32 - y0 as i32;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;

    for x in x0..x1+1 {
        if steep {
            image.put_pixel(y as u32, x, color);
        } else {
            image.put_pixel(x, y as u32, color);
        }
        error += derror;
        if error > dx {
            if y1 > y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error -= 2*dx;
        }
    }
}

fn main() {
    let mut im = ImageBuffer::new(512, 512);
    line(0, 0, 100, 100, &mut im, image::Rgb{data: [0, 0, 255]});
    line(80, 20, 100, 100, &mut im, image::Rgb{data: [0, 0, 255]});
    im.save("image.png").unwrap();
}
