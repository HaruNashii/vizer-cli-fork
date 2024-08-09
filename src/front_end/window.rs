use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};



//====================================//
//===============(WINDOW)=============//
//====================================//
// DISCLAIMER: DON'T CHANGE IT, IT WILL BROKE ALL THE ELEMENTS POSITION
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;





pub static mut EVENT_PUMP: Vec<sdl2::EventPump> = Vec::new();





pub fn create_window() -> (TextureCreator<WindowContext>, Canvas<Window>)
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("Media", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();
    let event_pump = sdl_started.event_pump().unwrap();

    unsafe{EVENT_PUMP.push(event_pump)};

    (texture_creator, canvas)
}



pub fn render_scene(text_vector: Vec<Texture>, rect_vector: Vec<Rect>, canvas: &mut Canvas<Window>)
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if !rect_vector.is_empty()
    {
        let mut index = 0;
        for rect in rect_vector
        {
            canvas.copy(&text_vector[index], None, rect).unwrap();
            index += 1;
        };
    };

    canvas.present();
}
