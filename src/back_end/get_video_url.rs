use fantoccini::{Client, Locator};

use crate::back_end::click_element::click_element;
use crate::full_stack::language::Translations;





pub async fn get_url(driver: &Client, language: &Translations) -> String
{
    // NOTE: We have to use two enter_frame instead of enter_frame(1)
    // because the video don't load fast enough
    // and we need to specify which frame to enter
    // because firefox handles iframes differently from chromium
    driver.wait().for_element(Locator::Css("iframe[src^='embed']")).await.unwrap();
    let player_div = driver.find(Locator::Css("iframe[src^='embed']")).await.unwrap();
    player_div.enter_frame().await.unwrap();

    driver.wait().for_element(Locator::Css("iframe")).await.unwrap();
    driver.enter_frame(Some(0)).await.unwrap();
    // we wait for the elements inside the iframe to appear
    driver.wait().for_element(Locator::Css("video")).await.unwrap();
    driver.wait().for_element(Locator::Id("videojs")).await.unwrap();
    driver.wait().for_element(Locator::Css(".vjs-big-play-button")).await.unwrap();

    // NOTE: We use this loop to ensure that we click on the button so that the src of the video appears
    loop 
    {
        match driver.find(Locator::Css("video[src]")).await 
        {
            Ok(_) => { break; }
            Err(_) => 
            {
                // NOTE: We find the button here so we don't get the
                // StaleElementReferenceException error
                let play_button = driver.find(Locator::Css(".vjs-big-play-button")).await;
                click_element(driver, play_button.unwrap().clone(), language.click_play_button_err).await;
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    }

    let video = driver.find(Locator::Css("video[src]")).await.unwrap();
    let video_src = video.attr("src").await.unwrap();

    // NOTE: We use this to leave the iframes so we can do some action later
    driver.enter_parent_frame().await.unwrap();
    driver.enter_parent_frame().await.unwrap();

    let video_url = format!("https:{}", video_src.unwrap());

    video_url
}
