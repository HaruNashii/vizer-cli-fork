use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::thread::sleep;

use fantoccini::{Client, ClientBuilder};

use crate::USE_GECKODRIVER;





async fn get_driver() -> Client 
{
    let use_geckodriver = USE_GECKODRIVER.get().unwrap();
    let driver: Client = if *use_geckodriver 
    {
        let mut caps = serde_json::map::Map::new();
        let opts = serde_json::json!({
            "prefs": {
                "media.autoplay.default": 1,
                "media.autoplay.allow-muted": false,
                "media.autoplay.blocking_policy": 2,
            },
            "args": ["--headless"]
        });

        caps.insert("moz:firefoxOptions".to_string(), opts);
        ClientBuilder::native().capabilities(caps).connect("http://localhost:4444").await.expect("failed to connect to WebDriver")
    } 
    else 
    {
        let mut caps = serde_json::map::Map::new();
        let opts = serde_json::json!
        ({
            "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
            "excludeSwitches": ["enable-logging"]
        });
        caps.insert("goog:chromeOptions".to_string(), opts);
        ClientBuilder::native().capabilities(caps).connect("http://localhost:9515").await.expect("failed to connect to WebDriver")
    };

    driver
}



fn start_browser_driver() -> Child 
{
    let use_geckodriver = USE_GECKODRIVER.get().unwrap();
    let driver_command = if *use_geckodriver 
    {
        "geckodriver"
    }
    else 
    {
        "chromedriver"
    };

    let browser_driver = Command::new(driver_command).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();

    sleep(Duration::from_millis(100));

    browser_driver
}



pub async fn start_drivers() -> (Client, Child)
{
    let client_to_send = get_driver();
    let child_to_send = start_browser_driver();

    (client_to_send.await, child_to_send)
}
