use std::
{
    sync::OnceLock,
    thread,
    time::Duration,
};


use crate::back_end::
{
    play_video::play_video,
    is_offline::is_offline,
    media::{search_media, select_media},
    get_video_url::get_url,
    start_driver::start_drivers,
    season::select_season,
    episode::{select_episode, select_episode_language},
};
    
use crate::full_stack::
{
    sdl_events::quit,
    language::{get_translation, Translations},
};

use crate::front_end::
{   
    ui::fonts,
    window::{create_window, render_scene, EVENT_PUMP},
};





mod back_end;
mod front_end;
mod full_stack;





static TRANSLATION: OnceLock<Translations> = OnceLock::new();
static USE_MPV: OnceLock<bool> = OnceLock::new();
static USE_GECKODRIVER: OnceLock<bool> = OnceLock::new();


// GLOBAL STRINGS USED BY THE FRONT-END
pub static mut RENDER_STAGE: u8 = 1;






#[tokio::main]
async fn main() 
{
    //===============================================================================================================//
    //----------------------------------------------SETUP THE BACK-END-----------------------------------------------//
    //===============================================================================================================//
    USE_MPV.get_or_init(|| true);
    USE_GECKODRIVER.get_or_init(|| true);
    TRANSLATION.get_or_init(|| get_translation("english"));
    let language = TRANSLATION.get().unwrap();
    


    //===============================================================================================================//
    //---------------------------------------------CHECK IF ITS ONLINE-----------------------------------------------//
    //===============================================================================================================//
    if is_offline().await { panic!("{}", language.is_currently_offline);}



    //===============================================================================================================//
    //------------------------------------------------START FRONT-END------------------------------------------------//
    //===============================================================================================================//

        thread::spawn
        (move || {
            let (texture_creator, mut canvas) = create_window(); 

            loop
            {
                std::thread::sleep(Duration::from_millis(32));
                //===============================================================================================================//
                //------------------------------------------------GET THE UI DATA------------------------------------------------//
                //===============================================================================================================//
                let (text_image_vector, text_rect_vector) = fonts(&texture_creator);
            


                //===============================================================================================================//
                //------------------------------------------------RENDER THE SCENE-----------------------------------------------//
                //===============================================================================================================//
                render_scene(text_image_vector, text_rect_vector, &mut canvas);
            };
        });



        //===============================================================================================================//
        //-----------------------------------------------------GET EVENT PUMP--------------------------------------------//
        //===============================================================================================================//
        while unsafe{EVENT_PUMP.is_empty()} { std::thread::sleep(Duration::from_millis(32)); };
        let mut event_pump = unsafe{EVENT_PUMP.remove(0)};
   

            
            //===============================================================================================================//
            //-----------------------------------------------------START DRIVER----------------------------------------------//
            //===============================================================================================================//
            let (driver, mut browser_driver) = start_drivers().await;
   


            //===============================================================================================================//
            //------------------------------------------------SEARCH FOR MEDIA-----------------------------------------------//
            //===============================================================================================================//
            unsafe{RENDER_STAGE = 1};
            let medias = search_media(&language, &mut event_pump).await;


            //===============================================================================================================//
            //------------------------------------------------SELECT THE MEDIA----------------------------------------------//
            //===============================================================================================================//
            unsafe{RENDER_STAGE = 2};
            let media_selected = select_media(medias, &mut event_pump).await;
            


            //===============================================================================================================//
            //-----------------------------------------------------SETUP DRIVER----------------------------------------------//
            //===============================================================================================================//
            let url = format!("https://vizer.in/{}", media_selected.url);
            driver.goto(&url).await.unwrap();
            if media_selected.url.contains("serie/") 
            {



                //===============================================================================================================//
                //------------------------------------------------SELECT THE SEASON----------------------------------------------//
                //===============================================================================================================//
                select_season(language, &driver).await;
                
                   
                //===============================================================================================================//
                //------------------------------------------------SELECT THE EPISODE---------------------------------------------//
                //===============================================================================================================//
                unsafe{RENDER_STAGE = 3};
                select_episode(language, &driver, &mut event_pump).await;
            };


             
            //===============================================================================================================//
            //------------------------------------------------SELECT THE EPISODE LANGUAGE------------------------------------//
            //===============================================================================================================//
            unsafe{RENDER_STAGE = 4};
            select_episode_language(language, &driver, &mut event_pump).await;



            //===============================================================================================================//
            //------------------------------------------------GET VIDEO URL--------------------------------------------------//
            //===============================================================================================================//
            unsafe{RENDER_STAGE = 5};
            let video_url = get_url(&driver, language).await;
            


            //===============================================================================================================//
            //------------------------------------------------PLAY THE VIDEO-------------------------------------------------//
            //===============================================================================================================//
            play_video(&video_url);



            //===============================================================================================================//
            //------------------------------------------------CLOSE THE DRIVER-----------------------------------------------//
            //===============================================================================================================//
            driver.close().await.unwrap();
            browser_driver.kill().unwrap();
            


            //===============================================================================================================//
            //------------------------------------------------HANDLER THE EXIT-----------------------------------------------//
            //===============================================================================================================//
            quit(&mut event_pump);
}
