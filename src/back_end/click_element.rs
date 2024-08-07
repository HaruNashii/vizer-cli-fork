use fantoccini::{elements::Element, Client};
use serde_json::json;





pub async fn click_element(driver: &Client, element: Element, error_message: &str) 
{
    driver.execute
    (
        r#"
        arguments[0].click();
        "#,
        vec![json!(element)],
    ).await.expect(error_message);
}
