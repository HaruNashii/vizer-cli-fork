use std::sync::OnceLock;
use fantoccini::Locator;

use crate::back_end::
{
    play_video::play_video,
    is_offline::is_offline,
    get_medias::get_medias,
    get_video_url::get_url,
    start_driver::{get_driver, start_browser_driver},
    click_element::click_element,
    season::parse_seasons,
    episode::parse_episodes,
};
    
use crate::full_stack::
{
    sdl_events::{search, quit, choose},
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
    let (mut canvas, _texture_creator, sdl_started) = create_window();
    let mut event_pump = sdl_started.event_pump().unwrap();
   


    //===============================================================================================================//
    //------------------------------------------------SEARCH FOR MEDIA-----------------------------------------------//
    //===============================================================================================================//
    let media_name = search(&mut event_pump);
    let medias = get_medias(&media_name).await;
    let media_url = &medias[0].url;
    if medias.is_empty() 
    {
        panic!("{}", language.media_name_is_empty_exit_text); 
    }



    //===============================================================================================================//
    //-----------------------------------------------------SETUP DRIVER----------------------------------------------//
    //===============================================================================================================//
    let mut browser_driver = start_browser_driver();
    let driver = get_driver().await;
    let url = format!("https://vizer.in/{}", &media_url);
    driver.goto(&url).await.unwrap();
                   


    if media_url.contains("serie/") 
    {
        //===============================================================================================================//
        //------------------------------------------------SELECT THE SEASON----------------------------------------------//
        //===============================================================================================================//
        let seasons = parse_seasons(&driver).await;
        let season_opts: Vec<&str> = seasons.iter().map(|s| s.text.as_str()).collect();
        let season_selected = choose(season_opts.len(), &mut event_pump);
        seasons[season_selected].clone().click_season(&driver, language.click_season_err).await;

        
           
        //===============================================================================================================//
        //------------------------------------------------SELECT THE EPISODE---------------------------------------------//
        //===============================================================================================================//
        println!("{}", language.select_episode_misc_text);
        let episodes = parse_episodes(&driver).await;
        let episode_opts: Vec<&str> = episodes.iter().map(|s| s.text.as_str()).collect();
        let episode_selected = choose(episode_opts.len(), &mut event_pump);
        println!("\n number of episodes = {}, \n episode selected = {} \n", episode_opts.len(), episode_opts[episode_selected].to_string());
        episodes[episode_selected].clone().click_episode(&driver, language.click_episode_err).await;
    };


     
    //===============================================================================================================//
    //------------------------------------------------SELECT THE LANGUAGE--------------------------------------------//
    //===============================================================================================================//
    println!("{}", language.getting_language_misc_text);
    driver.wait().for_element(Locator::Css("div[data-audio]")).await.unwrap();

    let langs_items = driver.find_all(Locator::Css("div[data-audio]")).await.unwrap();
    let mut langs_opts: Vec<String> = Vec::new();
    for lang in &langs_items 
    {
        let opt = lang.attr("data-audio").await.expect(language.language_option_expect);
        langs_opts.push(opt.unwrap());
    }

    let lang_opt = if langs_opts.len() == 2 
    {
        let lang_selected = choose(langs_opts.len(), &mut event_pump);
        println!("\n number of lang = {}, \n media selected = {} \n", langs_opts.len(), langs_opts[lang_selected].to_string());
        langs_opts[lang_selected].to_string()
    }
    else 
    {
        langs_opts[0].to_string()
    };

    for lang in langs_opts 
    {
        if lang == lang_opt 
        {
            let lang_css_selector = format!("div[data-audio='{}']", lang_opt);
            driver.find(Locator::Css(&lang_css_selector)).await.unwrap().click().await.unwrap();
            break;
        }
    }
    
    let mixdrop_btn = driver.find(Locator::Css("div[data-load-embed-server='mixdrop']")).await.unwrap();
    click_element(&driver, mixdrop_btn, language.language_option_expect).await;



    //===============================================================================================================//
    //------------------------------------------------PLAY THE VIDEO-------------------------------------------------//
    //===============================================================================================================//
    let video_url = get_url(&driver, language).await;
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
