fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;

    for i in 0..max_iter {
        let re = z_re * z_re - z_im * z_im + c_re;
        let im = 2.0 * z_re * z_im + c_im;

        z_re = re;
        z_im = im;

        if z_re * z_re + z_im * z_im > 4.0 {
            return i;
        }
    }

    max_iter
}

fn main() {
    let width = 80;
    let height = 40;
    let max_iter = 100;

    for y in 0..height {
        for x in 0..width {
            let c_re = (x as f64 / width as f64) * 3.5 - 2.5;
            let c_im = (y as f64 / height as f64) * 2.0 - 1.0;

            let iter = mandelbrot(c_re, c_im, max_iter);

            let ch = match iter {
                0..=10 => ' ',
                11..=30 => '.',
                31..=60 => '*',
                _ => '#',
            };

            print!("{}", ch);
        }
        println!();
    }
}
