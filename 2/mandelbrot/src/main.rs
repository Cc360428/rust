use num::Complex;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
}

fn complex_square_add_loop(c: Complex<f64>) -> Complex<f64> {
    /*
        这一行计算复数 z 的平方，并加上参数 c。这个操作会不断更新 z 的值。
        这通常用于计算某些数学序列或迭代，常见于分形图形的生成（如曼德尔布罗集）。
    */
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None
        }
    }
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}


#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
    Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
    upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width as f64,
        im: upper_left.im - pixel.1 as f64 * height as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100),
    (25, 75),
    Complex { re: -1.0, im: 1.0 },
    Complex { re: 1.0, im: -1.0 }),
    Complex { re: -0.5, im: -0.5 });
}

fn render(pixels: &mut [u8],bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row),
            upper_left, lower_right);
        }
        pixels[row * bounds.0 + col] = 255;
        match escape_time(point, 255) {
            None =>0,
            Some(count) => 255 - count as u8
        }
    }
}

fn main() {
    complex_square_add_loop();
    println!("Hello, world!");
}
