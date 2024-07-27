use crate::episode::{choose_episode, parse_episodes, Episode};
use crate::season::{choose_season, parse_seasons, Season};
use crate::media::{get_medias, choose_media, get_video_url, get_media_url, get_media_name_from_user};
use crate::player::watch_media::play_video;
use crate::quit::quit;
use crate::get_posters_path;
use crate::TRANSLATION;

use std::process::Child;
use selthi::Select;
use thirtyfour::WebDriver;



pub fn menu<'a>(menu_options: Vec<&'a str>, message: &'a str) -> Result<&'a str, ()> {
    let ans = Select::new(message, menu_options)
        .without_help_message()
        .prompt();

    match ans {
        Some(option) => Ok(option),
        None => Err(println!("Couldn't get option!")),
    }
}



pub fn get_menu_message<'a>(media_name: &'a str, episodes: &'a [Episode], current_episode: usize) -> String {
    let language = TRANSLATION.get().unwrap();

    if episodes.is_empty() {
        // Will return this message when its a movie
        return format!("{} {}", language.menu_msg_playing, media_name);
    }

    // HACK: We use the format! like that as a workaround because
    // its not possible to use format! with a variable instead of a string literal
    // where the language.menu_message its something like this:
    // "Playing episode {} of {} ({} episodes)"
    format!(
        "{} {} {} {} {} ({} {})",
        language.menu_msg_playing,
        language.menu_msg_episode,
        current_episode + 1,
        language.menu_msg_of,
        media_name,
        episodes.len(),
        language.menu_msg_episodes,
    )
}



pub fn get_menu_options(seasons: &[Season], episodes: &[Episode], current_episode: usize) -> Vec<&'static str> {
    if seasons.is_empty() {
        // Will return this options when its a movie
        return vec!["replay", "search", "quit"];
    }
    let mut menu_options: Vec<&str> = Vec::new();

    let first_episode = episodes.first().unwrap();
    let last_episode = episodes.last().unwrap();

    let is_last_episode = current_episode == last_episode.episode_number;
    let is_first_episode = current_episode == first_episode.episode_number;

    let is_just_one_episode = episodes.len() == 1;
    let is_just_one_season = seasons.len() == 1;

    if !is_last_episode {
        menu_options.push("next");
    }

    menu_options.push("replay");

    if !is_first_episode {
        menu_options.push("previous");
    }

    menu_options.push("search");

    if !is_just_one_episode {
        menu_options.push("select episode");
    }

    if !is_just_one_season {
        menu_options.push("select season");
    }

    menu_options.push("quit");

    menu_options
}


pub async fn menu_actions(browser_driver: Child, driver: WebDriver, img_mode: bool, mut video_url: String, mut seasons: Vec<Season>, mut episodes: Vec<Episode>, mut current_season: usize, mut current_episode: usize, mut media_name: String) {
    let language = TRANSLATION.get().unwrap();

    loop {
        let menu_options = get_menu_options(&seasons, &episodes, current_episode);
        let message = get_menu_message(&media_name, &episodes, current_episode);

        match menu(menu_options, &message) {
            Ok("replay") => play_video(&video_url),

            Ok("quit") => break,

            Ok("next") => {
                let result_driver_back = driver.back().await;
                match result_driver_back {
                    Ok(_) => {},
                    Err(err) => quit(Some(driver.to_owned()), None, Some(&format!("driver back ERROR, err output = {}", err))).await,
                }

                let result_seasons_parse = parse_seasons(&driver).await;
                match result_seasons_parse {
                    Ok(result) => {seasons = result},
                    Err(err) => quit(Some(driver.to_owned()), None, Some(&format!("parse seasons ERROR, err output = {}", err))).await,
                }

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await.unwrap();

                println!("{}", language.getting_episodes_misc_text);
                episodes = parse_episodes(&driver, img_mode).await.unwrap();
                current_episode += 1;

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await.unwrap();

                let media_url = get_media_url(&driver).await.unwrap();
                video_url = get_video_url(&driver, media_url).await.unwrap();
                play_video(&video_url);
            }



            Ok("previous") => {
                driver.back().await.unwrap();
                seasons = parse_seasons(&driver).await.unwrap();

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await.unwrap();

                println!("{}", language.getting_episodes_misc_text);
                episodes = parse_episodes(&driver, img_mode).await.unwrap();
                current_episode -= 1;

                episodes[current_episode]
                    .clone()
                    .click_episode(&driver, language.click_episode_err)
                    .await.unwrap();

                let media_url = get_media_url(&driver).await.unwrap();
                video_url = get_video_url(&driver, media_url).await.unwrap();
                play_video(&video_url);
            }



            Ok("select episode") => {
                driver.back().await.unwrap();
                seasons = parse_seasons(&driver).await.unwrap();
                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await.unwrap();

                println!("{}", language.getting_episodes_misc_text);
                episodes = parse_episodes(&driver, img_mode).await.unwrap();
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
                    .await.unwrap();

                let media_url = get_media_url(&driver).await.unwrap();
                video_url = get_video_url(&driver, media_url).await.unwrap();
                play_video(&video_url);
            }



            Ok("select season") => {
                driver.back().await.unwrap();
                seasons = parse_seasons(&driver).await.unwrap();
                let season_opts: Vec<&str> = seasons.iter().map(|s| s.text.as_str()).collect();
                current_season = choose_season(season_opts.clone()).unwrap();

                seasons[current_season]
                    .clone()
                    .click_season(&driver, language.click_season_err)
                    .await.unwrap();

                println!("{}", language.getting_episodes_misc_text);
                episodes = parse_episodes(&driver, img_mode).await.unwrap();
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
                    .await.unwrap();

                let media_url = get_media_url(&driver).await.unwrap();
                video_url = get_video_url(&driver, media_url).await.unwrap();
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
                        driver.goto(url).await.unwrap();
                        media_name = media.title;

                        if media.url.contains("serie/") {
                            seasons = parse_seasons(&driver).await.unwrap();
                            let season_opts: Vec<&str> = seasons.iter().map(|s| s.text.as_str()).collect();

                            current_season = if season_opts.len() > 1 {
                                choose_season(season_opts.clone()).unwrap()
                            } else {
                                0
                            };

                            seasons[current_season]
                                .clone()
                                .click_season(&driver, language.click_season_err)
                                .await.unwrap();

                            println!("{}", language.getting_episodes_misc_text);
                            episodes = parse_episodes(&driver, img_mode).await.unwrap();
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
                                .await.unwrap();
                        } else {
                            seasons.clear();
                            episodes.clear();
                        }

                        let media_url = get_media_url(&driver).await.unwrap();
                        video_url = get_video_url(&driver, media_url).await.unwrap();
                        play_video(&video_url);
                    }     
                    Err(_) => break,
                 }
            }
            Err(_) => break,
            _ => break,

        }
    }

    quit(Some(driver), Some(browser_driver), None).await;
}
