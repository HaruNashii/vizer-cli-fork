use thirtyfour::{DesiredCapabilities, WebDriver};
use crate::USE_GECKODRIVER;
use std::{
    process::{Child, Command, Stdio},
    thread::sleep,
    time::Duration,
};



pub fn kill_browser_driver() {
    let use_geckodriver = USE_GECKODRIVER.get().unwrap();

    if *use_geckodriver {
        let mut gecko_child = Command::new("pkill").arg("geckodriver").spawn().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        gecko_child.kill().unwrap();
    } else {
        let mut chrome_child = Command::new("pkill").arg("chromedriver").spawn().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        chrome_child.kill().unwrap();
    }

}



pub async fn get_driver() -> WebDriver {
    let use_geckodriver = USE_GECKODRIVER.get().unwrap();

    let driver: WebDriver = if *use_geckodriver {
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless().unwrap();
        WebDriver::new("http://localhost:4444", caps).await.unwrap()
    } else {
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless().unwrap();
        WebDriver::new("http://localhost:9515", caps).await.unwrap()
    };

    driver
}



pub fn start_browser_driver() -> Child {
    let use_geckodriver = USE_GECKODRIVER.get().unwrap();
    let driver_command = if *use_geckodriver {
        "geckodriver"
    } else {
        "chromedriver"
    };

    let browser_driver = Command::new(driver_command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    // we need to wait command to start :(
    sleep(Duration::from_millis(200));

    browser_driver
}
