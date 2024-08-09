use core::fmt;
use scraper::{Html, Selector};


use crate::full_stack::
{
    sdl_events::{search, choose},
    language::Translations,
};

use crate::
{
    TRANSLATION,
};

use crate::front_end::ui::
{
    MEDIA_OPTIONS,
    MEDIA_SELECTED,
};

#[derive(Clone)]
pub struct Media 
{
    pub title: String,
    pub url: String,
    pub poster_url: String,
}



impl fmt::Display for Media 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!
        (
            f,
            "Title - {} ; Link - {} ; Poster Url - {}",
            self.title, self.url, self.poster_url
        )
    }
}


 
pub async fn get_medias(media_name: String) -> Vec<Media> 
{
    let language = TRANSLATION.get().unwrap();

    let url = format!("https://vizer.in/pesquisar/{}", media_name);
    let response = reqwest::get(url).await.expect(language.response_expect);
    let html = response.text().await.unwrap();
    let document = Html::parse_document(html.as_str());

    let list_posters_selector = Selector::parse(r#"div[class="listItems"]"#).unwrap();
    let img_selector = Selector::parse(r#"img[class="img"]"#).unwrap();
    let poster_selector = Selector::parse("a").unwrap();

    let list_media = document.select(&list_posters_selector).next().unwrap();
    let mut medias = Vec::new();

    for (poster, img) in list_media.select(&poster_selector).zip(list_media.select(&img_selector))
    {
        let media_title = poster.value().attr("title").unwrap().replace("Assistir ", "").replace(" online", "");
        let media_url = poster.value().attr("href").unwrap();
        let img_src = img.value().attr("src").unwrap();
        let img_url = format!("https://vizertv.in{}", img_src.replace("t/185", "t/342"));

        let media = Media 
        {
            title: media_title,
            url: media_url.to_string(),
            poster_url: img_url,
        };

        medias.push(media);
    }

    medias
}



pub async fn search_media(language: &Translations, event_pump: &mut sdl2::EventPump) -> Vec<Media>
{
    //println!("{}", language.preparing_misc_text);

    let medias_searched = search(event_pump);
    let medias = get_medias(medias_searched).await;
    if medias.is_empty() 
    {
        panic!("{}", language.media_name_is_empty_exit_text); 
    }


    medias
}



pub async fn select_media(medias: Vec<Media>, event_pump: &mut sdl2::EventPump) -> Media {
    unsafe
    {
        for media in &medias 
        {
                MEDIA_OPTIONS.push(media.title.clone());
        };
        //println!("\n =========# ALL MEDIAS #============== \n {:?} \n =============================== \n", MEDIA_OPTIONS);
    }


    //println!("{}", language.select_media_misc_text);
    let choosed_media = choose(medias.len(), event_pump);
    unsafe 
    {
        MEDIA_SELECTED = medias[choosed_media].title.clone();
        //println!("\n =========# SELECTED MEDIA #============== \n {} \n =============================== \n", MEDIA_SELECTED);
    };

    medias[choosed_media.clone()].clone()
}
