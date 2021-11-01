use crate::tga_image::TGAImage;
mod tga_image;

fn main() {
    let mut image = TGAImage::new(100, 100, tga_image::BLACK_COLOR);
    image.set_pixel(10, 80, tga_image::RED_COLOR);
    image.write_tga("output.tga");
    println!("Done 🏁");
}
