use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};
use crate::quit::quit;

/// This function is used when we need to click on a web element but there is a pop up in front of
/// it
pub async fn click_element(driver: &WebDriver, element: WebElement, error_message: &str) -> WebDriverResult<()> {
        let result = driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![element.to_json()?],
        )
        .await;

    match result {
        Ok(_) => {},
        Err(_) => quit(Some(driver.to_owned()), None, Some(error_message)).await,
    }

    Ok(())
}
