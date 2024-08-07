use std::sync::OnceLock;

use crate::back_end::
{
    play_video::play_video,
    is_offline::is_offline,
    media::search_media,
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

use crate::front_end::window::{create_window, render_scene};





mod back_end;
mod front_end;
mod full_stack;





static TRANSLATION: OnceLock<Translations> = OnceLock::new();
static USE_MPV: OnceLock<bool> = OnceLock::new();
static USE_GECKODRIVER: OnceLock<bool> = OnceLock::new();





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
    if is_offline().await 
    {
        panic!("{}", language.is_currently_offline);
    }



    //===============================================================================================================//
    //------------------------------------------------START FRONT-END------------------------------------------------//
    //===============================================================================================================//
    let (mut canvas, _texture_creator, mut event_pump) = create_window();
   


    //===============================================================================================================//
    //------------------------------------------------SEARCH FOR MEDIA-----------------------------------------------//
    //===============================================================================================================//
    let medias = search_media(&language, &mut event_pump).await;



    //===============================================================================================================//
    //-----------------------------------------------------START DRIVER----------------------------------------------//
    //===============================================================================================================//
    let (driver, mut browser_driver) = start_drivers().await;



    //===============================================================================================================//
    //-----------------------------------------------------SETUP DRIVER----------------------------------------------//
    //===============================================================================================================//
    let url = format!("https://vizer.in/{}", &medias[0].url);
    driver.goto(&url).await.unwrap();
    if medias[0].url.contains("serie/") 
    {
        //===============================================================================================================//
        //------------------------------------------------SELECT THE SEASON----------------------------------------------//
        //===============================================================================================================//
        select_season(language, &driver, &mut event_pump).await;
    
        
           
        //===============================================================================================================//
        //------------------------------------------------SELECT THE EPISODE---------------------------------------------//
        //===============================================================================================================//
        select_episode(language, &driver, &mut event_pump).await;
    };


     
    //===============================================================================================================//
    //------------------------------------------------SELECT THE EPISODE LANGUAGE------------------------------------//
    //===============================================================================================================//
    select_episode_language(language, &driver, &mut event_pump).await;



    //===============================================================================================================//
    //------------------------------------------------GET VIDEO URL--------------------------------------------------//
    //===============================================================================================================//
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



    loop 
    {
        //===============================================================================================================//
        //------------------------------------------------RENDER THE SCENE-----------------------------------------------//
        //===============================================================================================================//
        render_scene(&mut canvas);
        


        //===============================================================================================================//
        //------------------------------------------------HANDLER THE EXIT-----------------------------------------------//
        //===============================================================================================================//
        quit(&mut event_pump);
    };
}
