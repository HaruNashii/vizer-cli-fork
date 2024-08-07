use fantoccini::{Client, Locator};

use crate::back_end::click_element::click_element;
use crate::full_stack::language::Translations;





pub async fn get_url(driver: &Client, language: &Translations) -> String
{
    driver.wait().for_element(Locator::Css("iframe[src^='embed']")).await.unwrap();
    let player_div = driver.find(Locator::Css("iframe[src^='embed']")).await.unwrap();
    player_div.enter_frame().await.unwrap();

    driver.wait().for_element(Locator::Css("iframe")).await.unwrap();
    driver.enter_frame(Some(0)).await.unwrap();
    driver.wait().for_element(Locator::Css("video")).await.unwrap();
    driver.wait().for_element(Locator::Id("videojs")).await.unwrap();
    driver.wait().for_element(Locator::Css(".vjs-big-play-button")).await.unwrap();

    loop 
    {
        match driver.find(Locator::Css("video[src]")).await 
        {
            Ok(_) => { break; }
            Err(_) => 
            {
                let play_button = driver.find(Locator::Css(".vjs-big-play-button")).await;
                click_element(driver, play_button.unwrap().clone(), language.click_play_button_err).await;
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    }

    let video = driver.find(Locator::Css("video[src]")).await.unwrap();
    let video_src = video.attr("src").await.unwrap();

    driver.enter_parent_frame().await.unwrap();
    driver.enter_parent_frame().await.unwrap();

    let video_url = format!("https:{}", video_src.unwrap());

    video_url
}
