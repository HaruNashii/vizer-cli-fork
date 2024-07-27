use thirtyfour::prelude::*;

use crate::{
    player::media_player::{open_mpv, open_vlc},
    season::{parse_seasons, choose_season},
    episode::{choose_episode, parse_episodes},
    media::{Media, get_video_url, get_media_url},
    cli::menu::menu_actions,
    driver::manage_driver::{kill_browser_driver, get_driver, start_browser_driver},

    USE_MPV,
    TRANSLATION,
};



pub fn play_video(video_url: &str) {
    let use_mpv = USE_MPV.get().unwrap();

    if *use_mpv {
        open_mpv(video_url);
    } else {
        open_vlc(video_url);
    }
}



pub async fn watch_media(media: Media, img_mode: bool) -> WebDriverResult<()> {
    // VARIABLE SETUP
    let language = TRANSLATION.get().unwrap();
    let mut seasons = Vec::new();
    let mut episodes = Vec::new();
    let mut current_season: usize = 0;
    let mut current_episode: usize = 0;
    let media_name: String = media.title;


    print!("\x1B[2J\x1B[1;1H");
    println!("{}", language.preparing_misc_text);


        // DRIVER SETUP
        kill_browser_driver();
        let browser_driver = start_browser_driver();
        let driver = get_driver().await;
        let url = format!("https://vizer.in/{}", &media.url);
        driver.goto(url).await?;


    if media.url.contains("serie/") {


            // SEASON SETUP
            {
                seasons = parse_seasons(&driver).await?;
                let season_opts: Vec<&str> = seasons.iter().map(|s| s.text.as_str()).collect();

                current_season = if season_opts.len() > 1 {
                    choose_season(season_opts.clone()).unwrap()
                } else {
                    0
                };

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await?;
            }


        println!("{}", language.getting_episodes_misc_text);


            // EPISODE SETUP
            {
                episodes = parse_episodes(&driver, img_mode).await?;
                let episode_opts: Vec<&str> = episodes.iter().map(|s| s.text.as_str()).collect();

                current_episode = if episode_opts.len() > 1 {

                    if episodes[0].img_path.is_some() {
                        let episodes_img_path = episodes
                            .iter()
                            .map(|i| i.img_path.as_ref().unwrap().as_str())
                            .collect();

                        choose_episode(episode_opts.clone(), Some(episodes_img_path)).unwrap()
                    } else {
                        choose_episode(episode_opts.clone(), None).unwrap()
                    }

                } else {
                    0
                };

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await?;
            }
    }


    // PLAY VIDEO SETUP
    let media_url = get_media_url(&driver).await?;
    let video_url = get_video_url(&driver, media_url).await?;
    play_video(&video_url);

    
    // MENU SETUP
    menu_actions(browser_driver, driver, img_mode, video_url, seasons, episodes, current_season, current_episode, media_name).await;


    Ok(())
}
