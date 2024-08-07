use fantoccini::{elements::Element, Client, Locator};

use crate::back_end::click_element::click_element;



#[derive(Clone)]
pub struct Episode 
{
    pub text: String,
    pub web_element: Element,
}

impl Episode 
{
    pub async fn click_episode(self, driver: &Client, error_message: &str) 
    {
        click_element(driver, self.web_element, error_message).await;
    }
}





pub async fn parse_episodes(driver: &Client) -> Vec<Episode> 
{

    let mut episodes = Vec::new();
    let mut list_of_episodes_text = Vec::new();
    let mut list_of_episodes_elements = Vec::new();
    

    let episodes_css_selector = ".episodes > div.item:not(.unreleased)";
    driver.wait().for_element(Locator::Css(episodes_css_selector)).await.unwrap();
    let episodes_items = driver.find_all(Locator::Css(episodes_css_selector)).await.unwrap();


    for (i, episode_element) in episodes_items.iter().enumerate() 
    {
        let episode_text = episode_element.find(Locator::Css("span")).await.unwrap().html(true).await.unwrap();
        let text: String = format!("{} - {}", i + 1, episode_text);

        list_of_episodes_text.push(text);
        list_of_episodes_elements.push(episode_element);
    }


    for episode in 0..list_of_episodes_elements.len() 
    {
        let episode = Episode 
        {
            text: list_of_episodes_text[episode].clone(),
            web_element: list_of_episodes_elements[episode].to_owned(),
        };

        episodes.push(episode);
    }


    episodes
}
