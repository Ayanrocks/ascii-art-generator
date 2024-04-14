use image;
use image::GenericImageView;

pub fn get_image(dir: &str) {
    let scale = 64;
    let img = image::open(dir).expect("invalid image url");
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();

    for i in 0..height {
        for j in 0..width {
            if i % (scale * 2) == 0 && j % scale == 0 {
                let pxl = img.get_pixel(j, i);
                let mut intent = pxl[0] / 3 + pxl[1] / 3 + pxl[2]/3;
                if pxl[3] == 0 {
                    intent = 0
                }
                print!("{}", get_ascii_char(intent))
            }
        }
        if i % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn get_ascii_char(intent: u8) -> &'static str {
    let idx = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[idx as usize];
}

