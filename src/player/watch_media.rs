use thirtyfour::prelude::*;

use crate::{
    cli::{
        choose_episode::choose_episode,
        choose_media::choose_media,
        choose_season::choose_season,
        get_media_name_from_user::get_media_name_from_user,
        get_media_url::get_media_url,
        get_medias::get_medias,
        get_video_url::get_video_url,
        menu::{get_menu_message, get_menu_options, menu},
    },
    driver::{
        parse_episodes::parse_episodes,
        parse_seasons::parse_seasons,
        start_driver::{get_driver, start_browser_driver},
    },
    fs::posters::get_posters_path,
    media::Media,
    player::play_video::play_video,
    TRANSLATION,
};

pub async fn watch_media(media: Media, img_mode: bool) -> WebDriverResult<()> {
    let language = TRANSLATION.get().unwrap();
    let mut seasons = Vec::new();
    let mut episodes = Vec::new();
    let mut current_season: usize = 0;
    let mut current_episode: usize = 0;
    let mut media_name: String = media.title;

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", language.preparing_misc_text);

    let mut browser_driver = start_browser_driver();
    let driver = get_driver().await;

    let url = format!("https://vizer.in/{}", &media.url);
    driver.goto(url).await?;

    if media.url.contains("serie/") {
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

        println!("{}", language.getting_episodes_misc_text);

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

    let media_url = get_media_url(&driver).await?;

    let mut video_url = get_video_url(&driver, media_url).await?;

    play_video(&video_url);

    loop {
        let menu_options = get_menu_options(&seasons, &episodes, current_episode);

        let message = get_menu_message(&media_name, &episodes, current_episode);

        match menu(menu_options, &message) {
            Ok("replay") => play_video(&video_url),
            Ok("quit") => break,
            Ok("next") => {
                driver.back().await?;

                seasons = parse_seasons(&driver).await?;

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await?;

                println!("{}", language.getting_episodes_misc_text);

                episodes = parse_episodes(&driver, img_mode).await?;
                current_episode += 1;

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await?;

                let media_url = get_media_url(&driver).await?;

                video_url = get_video_url(&driver, media_url).await?;

                play_video(&video_url);
            }
            Ok("previous") => {
                driver.back().await?;

                seasons = parse_seasons(&driver).await?;

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await?;

                println!("{}", language.getting_episodes_misc_text);

                episodes = parse_episodes(&driver, img_mode).await?;
                current_episode -= 1;

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await?;

                let media_url = get_media_url(&driver).await?;

                video_url = get_video_url(&driver, media_url).await?;

                play_video(&video_url);
            }
            Ok("select episode") => {
                driver.back().await?;

                seasons = parse_seasons(&driver).await?;

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await?;

                println!("{}", language.getting_episodes_misc_text);

                episodes = parse_episodes(&driver, img_mode).await?;

                let episode_opts: Vec<&str> = episodes.iter().map(|s| s.text.as_str()).collect();

                current_episode = if episodes[0].img_path.is_some() {
                    let episodes_img_path = episodes
                        .iter()
                        .map(|i| i.img_path.as_ref().unwrap().as_str())
                        .collect();

                    choose_episode(episode_opts.clone(), Some(episodes_img_path)).unwrap()
                } else {
                    choose_episode(episode_opts.clone(), None).unwrap()
                };

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await?;

                let media_url = get_media_url(&driver).await?;

                video_url = get_video_url(&driver, media_url).await?;

                play_video(&video_url);
            }
            Ok("select season") => {
                driver.back().await?;

                seasons = parse_seasons(&driver).await?;

                let season_opts: Vec<&str> = seasons.iter().map(|s| s.text.as_str()).collect();

                current_season = choose_season(season_opts.clone()).unwrap();

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await?;

                println!("{}", language.getting_episodes_misc_text);

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

                let media_url = get_media_url(&driver).await?;

                video_url = get_video_url(&driver, media_url).await?;

                play_video(&video_url);
            }
            Ok("search") => {
                let mut posters_path: Vec<String> = Vec::new();

                let media_name_from_user = get_media_name_from_user().unwrap();

                let medias = get_medias(&media_name_from_user).await;

                if medias.is_empty() {
                    eprintln!("{}", language.media_name_is_empty_exit_text);
                    break;
                }

                if img_mode {
                    let medias_poster_url: Vec<String> = medias
                        .clone()
                        .into_iter()
                        .map(|media| media.poster_url)
                        .collect();

                    posters_path = get_posters_path(medias_poster_url).await.unwrap();
                }
                match choose_media(medias, img_mode, posters_path) {
                    Ok(media) => {
                        let url = format!("https://vizer.in/{}", &media.url);
                        driver.goto(url).await?;

                        media_name = media.title;

                        if media.url.contains("serie/") {
                            seasons = parse_seasons(&driver).await?;

                            let season_opts: Vec<&str> =
                                seasons.iter().map(|s| s.text.as_str()).collect();

                            current_season = if season_opts.len() > 1 {
                                choose_season(season_opts.clone()).unwrap()
                            } else {
                                0
                            };

                            seasons[current_season]
                                .clone()
                                .click_season(&driver, language.click_season_err)
                                .await?;

                            println!("{}", language.getting_episodes_misc_text);

                            episodes = parse_episodes(&driver, img_mode).await?;

                            let episode_opts: Vec<&str> =
                                episodes.iter().map(|s| s.text.as_str()).collect();

                            current_episode = if episode_opts.len() > 1 {
                                if episodes[0].img_path.is_some() {
                                    let episodes_img_path = episodes
                                        .iter()
                                        .map(|i| i.img_path.as_ref().unwrap().as_str())
                                        .collect();

                                    choose_episode(episode_opts.clone(), Some(episodes_img_path))
                                        .unwrap()
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
                        } else {
                            seasons.clear();
                            episodes.clear();
                        }

                        let media_url = get_media_url(&driver).await?;

                        video_url = get_video_url(&driver, media_url).await?;

                        play_video(&video_url);
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        driver.to_owned().quit().await.unwrap();
                        browser_driver.kill().unwrap();
                        break;
                    }
                }
            }
            Err(err) => {
                eprint!("{:?}", err);
                driver.to_owned().quit().await.unwrap();
                browser_driver.kill().unwrap();
                break;
            }
            _ => break,
        }
    }

    driver.quit().await.unwrap();
    browser_driver.kill().unwrap();
    Ok(())
}
