use thirtyfour::WebDriver;
use std::process::Child;
use crate::{
    driver::manage_driver::kill_browser_driver,
    fs::temp_dir::remove_temp_dir,
};



pub async fn quit(browser_driver: Option<WebDriver>, driver: Option<Child>, err: Option<&str>) {

    match browser_driver {
        Some(browser_driver_result) => {
            let result_browser_driver_quit = browser_driver_result.quit().await;
            match result_browser_driver_quit {
                Ok(_) => {},
                Err(err) => println!("\n browser driver couldn't be quited ERROR, err output = {} \n", err),
            };
        },
        None => {},
    }

    match driver {
        Some(mut driver_result) => {
            let result_driver_kill = driver_result.kill();
            match result_driver_kill {
                Ok(_) => {},
                Err(err) => println!("\n driver couldn't be closed ERROR, err output = {} \n", err),
            }
        },
        None => {},
    }
    
    match err {
        Some(err) => println!("\n {} \n", err),
        None => {},
    }
    
    kill_browser_driver();
    remove_temp_dir();
    eprint!("\n See you next time! 0/");
    panic!("")
}
