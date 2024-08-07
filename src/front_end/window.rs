use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};





//====================================//
//===============(WINDOW)=============//
//====================================//
// DISCLAIMER: DON'T CHANGE IT, IT WILL BROKE ALL THE ELEMENTS POSITION
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;





pub fn create_window() -> (Canvas<Window>, TextureCreator<WindowContext>, sdl2::Sdl) 
{
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("Media", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    let texture_creator = canvas.texture_creator();

    (canvas, texture_creator, sdl_started)
}



pub fn render_scene(canvas: &mut Canvas<Window>) 
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();
}
