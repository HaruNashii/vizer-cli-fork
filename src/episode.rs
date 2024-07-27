use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};
use thirtyfour::prelude::*;
use selthi::Select;
use crate::driver::click_element::click_element;
use crate::fs::posters::get_posters_path;
use crate::{TRANSLATION, VIM_MODE};



#[derive(Clone)]
pub struct Episode {
    pub text: String,
    pub img_path: Option<String>,
    pub episode_number: usize,
    pub web_element: WebElement,
}

impl Episode {
    pub async fn click_episode(
        self,
        driver: &WebDriver,
        error_message: &str,
    ) -> WebDriverResult<()> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}



pub fn choose_episode(episodes: Vec<&str>, images_path: Option<Vec<&str>>) -> Result<usize, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    let help_msg = format!("{} {}", language.total_episode_misc_text, episodes.len());

    let ans = if let Some(images_path) = images_path {
        Select::new(language.select_episode_misc_text, episodes)
            .with_help_message(&help_msg)
            .with_page_size(24)
            .with_vim_mode(*vim_mode)
            .with_images(images_path)
            .prompt()
    } else {
        Select::new(language.select_episode_misc_text, episodes)
            .with_help_message(&help_msg)
            .with_page_size(24)
            .with_vim_mode(*vim_mode)
            .prompt()
    };

    match ans {
        Some(choice) => {
            let mut episode_number = choice.split_whitespace();
            let episode: usize = episode_number.next().unwrap().parse::<usize>().unwrap() - 0;
            Ok(episode)
        }
        None => Err(println!("{}", language.choose_episode_err)),
    }
}



pub async fn parse_episodes(driver: &WebDriver, img_mode: bool) -> WebDriverResult<Vec<Episode>> {
    let episodes_list = driver.find(By::ClassName("episodes")).await?;
    let episodes_items = episodes_list.query(By::ClassName("item")).all().await?;

    let mut episodes = Vec::new();
    let mut list_of_images_url = Vec::new();
    let mut list_of_episodes_text = Vec::new();
    let mut list_of_episodes_elements = Vec::new();

    for (i, episode_element) in episodes_items.iter().enumerate() {
        if episode_element.class_name().await?.unwrap() != "item unreleased " {
            let episode_text = episode_element
                .find(By::Tag("span"))
                .await?
                .inner_html()
                .await?;

            // this thing of adding by 1
            // is just to show the episodes starting in 1 instead of 0
            let text: String = format!("{} - {}", i + 1, episode_text);

            list_of_episodes_text.push(text);
            list_of_episodes_elements.push(episode_element);

            if img_mode {
                let img_src = episode_element
                    .find(By::Tag("img"))
                    .await?
                    .attr("src")
                    .await?
                    .unwrap();
                let img_url = format!("https://vizertv.in{}", img_src.replace("s/185", "s/500"));
                list_of_images_url.push(img_url);
            }
        } else {
            break;
        }
    }

    let poster_paths = if img_mode {
        Some(get_posters_path(list_of_images_url).await.unwrap())
    } else {
        None
    };

    for episode in 0..list_of_episodes_elements.len() {
        let episode = Episode {
            text: list_of_episodes_text[episode].clone(),
            img_path: poster_paths.as_ref().map(|paths| paths[episode].clone()),
            episode_number: episode,
            web_element: list_of_episodes_elements[episode].to_owned(),
        };
        episodes.push(episode);
    }

    Ok(episodes)
}
