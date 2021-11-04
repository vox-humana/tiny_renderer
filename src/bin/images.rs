use tiny_renderer::lessons::lessons;

pub fn main() {
    for lesson in lessons() {
        let image = (lesson.renderer)();
        let filename = format!("{}.tga", lesson.name);
        image.write_tga(filename);
    }
    println!("Done🏁");
}