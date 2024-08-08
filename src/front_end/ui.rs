use std::env;
use std::time::Duration;

//use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::full_stack::sdl_events::INPUT_TEXT;





// fonts
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);
const DEFAULT_FONT_SIZE: u16 = 20;

const INPUT_TEXT_FONT_POSITION: [i32; 2] = [200, 200];





fn get_exe_path() -> String {
    let mut current_path = String::new();
    match env::current_exe() {
        Ok(exe_path) => current_path.push_str(&exe_path.display().to_string()),
        Err(_) => println!("ERROR! Fail Getting Current Directory Path"),
    }
    if let Some(index) = current_path.rfind('/') {
        current_path.truncate(index + 1);
    };

    current_path
}



fn font_generator<'a>(additional_text: &str, texture_creator: &'a TextureCreator<WindowContext>, size: u16, text: String, x: i32, y: i32, ) -> (Texture<'a>, Rect) {
    let ttf_context = sdl2::ttf::init().unwrap();

    let exe_path = get_exe_path();
    let font_path = format!("{}{}", exe_path, DEFAULT_FONT_PATH);

    let font = ttf_context.load_font(font_path, size).unwrap();
    let surface = font.render(&format!("{}{}", additional_text, text)).blended(DEFAULT_FONT_COLOR).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let font_rect = Rect::new(x, y, surface.width(), surface.height());

    (texture, font_rect)
}



pub fn fonts(texture_creator: &TextureCreator<WindowContext>) -> (Vec<Texture>, Vec<Rect>) {
    std::thread::sleep(Duration::from_millis(250));

    let mut input_text = unsafe{INPUT_TEXT.clone()};
    if input_text.is_empty() 
    {
        input_text.push_str("is empty")
    }
    println!("\n ======================== \n input text = {} \n ======================== \n", input_text);


    let (input_text_image, input_text_rect) = font_generator(" ", texture_creator, DEFAULT_FONT_SIZE, input_text, INPUT_TEXT_FONT_POSITION[0], INPUT_TEXT_FONT_POSITION[1]);

    let text_vector = vec![input_text_image];
    let rect_vector = vec![input_text_rect];

    (text_vector, rect_vector)
}
