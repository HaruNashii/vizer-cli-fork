use fantoccini::{elements::Element, Client, Locator};

use crate::back_end::click_element::click_element;
use crate::full_stack::sdl_events::choose;
use crate::full_stack::language::Translations;


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



pub async fn select_episode(language: &Translations, driver: &Client, event_pump: &mut sdl2::EventPump) 
{
        println!("{}", language.select_episode_misc_text);
        let episodes = parse_episodes(&driver).await;
        let episode_opts: Vec<&str> = episodes.iter().map(|s| s.text.as_str()).collect();
        let episode_selected = choose(episode_opts.len(), event_pump);
        episodes[episode_selected].clone().click_episode(&driver, language.click_episode_err).await;
}



pub async fn select_episode_language(language: &Translations, driver: &Client, event_pump: &mut sdl2::EventPump)
{
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
        let lang_selected = choose(langs_opts.len(), event_pump);
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
}
