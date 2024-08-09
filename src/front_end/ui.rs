use std::env;

//use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::RENDER_STAGE;


// GLOBAL VARIABLE
pub static mut INPUT_TEXT: String = String::new();
pub static mut SELECTED_OPTION: u16 = 1;

pub static mut MEDIA_OPTIONS: Vec<String> = Vec::new();
pub static mut MEDIA_SELECTED: String = String::new();

pub static mut EPISODE_OPTIONS: Vec<String> = Vec::new();
pub static mut EPISODE_SELECTED: String = String::new();

pub static mut EPISODE_LANG_OPTIONS: Vec<String> = Vec::new();


// fonts
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);
const DEFAULT_FONT_SIZE: u16 = 20;
const SMALL_FONT_SIZE: u16 = 10;

const TEXT_PADDING: i32 = 30;

const QUIT_FONT_POSITION: [i32; 2] = [100, 250];

const SELECTED_OPTION_FONT_POSITION: [i32; 2] = [50, 50];
const TOTAL_OPTIONS_FONT_POSITION: [i32; 2] = [50, 100];

const INPUT_TEXT_FONT_POSITION: [i32; 2] = [50, 150];
const MEDIA_OPTIONS_FONT_POSITION: [i32; 2] = [200, 50];
const EPISODE_OPTIONS_FONT_POSITION: [i32; 2] = [200, 50];
const EPISODE_LANG_OPTIONS_FONT_POSITION: [i32; 2] = [200, 50];




fn get_exe_path() -> String {
    let mut current_path = String::new();
    match env::current_exe() {
        Ok(exe_path) => current_path.push_str(&exe_path.display().to_string()),
        Err(_) => panic!("ERROR! Fail Getting Current Directory Path"),
    }
    if let Some(index) = current_path.rfind('/') {
        current_path.truncate(index + 1);
    };

    current_path
}



fn font_generator<'a>(texture_creator: &'a TextureCreator<WindowContext>, additional_text: Option<&str>, text: String, size: u16, x: i32, y: i32, ) -> (Texture<'a>, Rect) {
    let ttf_context = sdl2::ttf::init().unwrap();
    
    let exe_path = get_exe_path();
    let font_path = format!("{}{}", exe_path, DEFAULT_FONT_PATH);

    let font = ttf_context.load_font(font_path, size).unwrap();
    match additional_text 
    {
        Some(some_text) => 
        {
            let surface = font.render(&format!("{}{}", some_text, text)).blended(DEFAULT_FONT_COLOR).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let font_rect = Rect::new(x, y, surface.width(), surface.height());

            return(texture, font_rect);
        },

        None => 
        {
            let surface = font.render(&text).blended(DEFAULT_FONT_COLOR).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            let font_rect = Rect::new(x, y, surface.width(), surface.height());

            return(texture, font_rect);
        },
    };
}



pub fn fonts(texture_creator: &TextureCreator<WindowContext>) -> (Vec<Texture>, Vec<Rect>) {
    let render_stage = unsafe{RENDER_STAGE};
    let selected_option = unsafe{SELECTED_OPTION};
    let input_text = unsafe{INPUT_TEXT.clone()};
    let media_options = unsafe{MEDIA_OPTIONS.clone()};
    let episode_options = unsafe{EPISODE_OPTIONS.clone()};
    let episode_lang_options = unsafe{EPISODE_LANG_OPTIONS.clone()};

    let mut text_vector = Vec::new();
    let mut rect_vector = Vec::new();

            
        // I FUCKING DON'T KNOW WHY THIS SHIT WON'T WORK IF I PUT IN THE SAME IF, THIS DON'T MAKE
        // ANY SENSE, I SPEND 9 FUCKIN HOURS THINKING THINGS THAT DON'T MAKE SENSE, RUST UNSAFE IS
        // ABSOLUTE HELL, THAT MAKES NO SENSE, PROGRAMMING IN RUST UNSAFE IS PURE RNG, THINGS WILL
        // WORK ONLY IF YOU HAVE LUCK, BECAUSE DON'T EXIST LOGIC FOR WHY THIS IS HAPPENING.
        if render_stage <= 4
        {
            let (input_text_image, input_text_rect) = font_generator(texture_creator, Some("Search = "), input_text.clone(), SMALL_FONT_SIZE, INPUT_TEXT_FONT_POSITION[0], INPUT_TEXT_FONT_POSITION[1]);
            text_vector.push(input_text_image);
            rect_vector.push(input_text_rect);
        };

        if render_stage >= 2 && render_stage <= 4
        {
            let (selected_option_image, selected_option_rect) = font_generator(texture_creator, Some("Selecting = "), selected_option.to_string(), SMALL_FONT_SIZE, SELECTED_OPTION_FONT_POSITION[0], SELECTED_OPTION_FONT_POSITION[1]);
            text_vector.push(selected_option_image);
            rect_vector.push(selected_option_rect);
        };


        match render_stage 
        {
            2 =>
            {
                if !media_options.is_empty()
                {
                    let (total_media_image, total_media_rect) = font_generator(texture_creator, Some("Total = "), media_options.len().to_string(), SMALL_FONT_SIZE, TOTAL_OPTIONS_FONT_POSITION[0], TOTAL_OPTIONS_FONT_POSITION[1]);
                    text_vector.push(total_media_image);
                    rect_vector.push(total_media_rect);

                    let mut index = 0;
                    for media_option in media_options
                    {
                        let (medias_options_image, medias_options_rect) = font_generator(texture_creator, Some(&format!("{} - ", index + 1)), media_option, DEFAULT_FONT_SIZE, MEDIA_OPTIONS_FONT_POSITION[0], MEDIA_OPTIONS_FONT_POSITION[1] + (TEXT_PADDING * index as i32));
                        text_vector.push(medias_options_image);
                        rect_vector.push(medias_options_rect);
                        index += 1;
                    };
                }
            },

            3 => 
            {
                if !episode_options.is_empty()
                {
                    let (total_episode_image, total_episode_rect) = font_generator(texture_creator, Some("Total = "), episode_options.len().to_string(), SMALL_FONT_SIZE, TOTAL_OPTIONS_FONT_POSITION[0], TOTAL_OPTIONS_FONT_POSITION[1]);
                    text_vector.push(total_episode_image);
                    rect_vector.push(total_episode_rect);

                    let mut index = 0;
                    for episode_option in episode_options
                    {
                        let (episode_options_image, episode_options_rect) = font_generator(texture_creator, None, episode_option, DEFAULT_FONT_SIZE, EPISODE_OPTIONS_FONT_POSITION[0], EPISODE_OPTIONS_FONT_POSITION[1] + (TEXT_PADDING * index as i32));
                        text_vector.push(episode_options_image);
                        rect_vector.push(episode_options_rect);
                        index += 1;
                    };
                };
            },

            4 =>
            {
                    let (total_episode_lang_image, total_episode_lang_rect) = font_generator(texture_creator, Some("Total = "), episode_lang_options.len().to_string(), SMALL_FONT_SIZE, TOTAL_OPTIONS_FONT_POSITION[0], TOTAL_OPTIONS_FONT_POSITION[1]);
                    text_vector.push(total_episode_lang_image);
                    rect_vector.push(total_episode_lang_rect);

                    let mut index = 0;
                    for lang_option in episode_lang_options
                    {
                        let (lang_options_image, lang_options_rect) = font_generator(texture_creator, Some(&format!("{} - ", index + 1)), lang_option, DEFAULT_FONT_SIZE, EPISODE_LANG_OPTIONS_FONT_POSITION[0], EPISODE_LANG_OPTIONS_FONT_POSITION[1] + (TEXT_PADDING * index as i32));
                        text_vector.push(lang_options_image);
                        rect_vector.push(lang_options_rect);
                        index += 1;
                    };
            },

            5 =>
            {
                let (quit_image, quit_rect) = font_generator(texture_creator, Some("Thanks For Using This App!"), String::from(" / You Can Quit Now >:3"), DEFAULT_FONT_SIZE, QUIT_FONT_POSITION[0], QUIT_FONT_POSITION[1]);
                text_vector.push(quit_image);
                rect_vector.push(quit_rect);
            },

            _=> {},
        };
    return (text_vector, rect_vector);
}
