use crate::helpers::spawn_app;
use newsletter_api::models::PublicProfileListItem;

#[tokio::test]
async fn returns_list_of_existing_users() {
    let app = spawn_app().await;

    let response = app.get_users().await;
    assert_eq!(200, response.status().as_u16());

    let response_body: Vec<PublicProfileListItem> = response.json().await.unwrap();

    assert_eq!(response_body.len(), 1);
    assert_eq!(&response_body[0].username, &app.test_user.username);
    assert_eq!(&response_body[0].description, "");
    assert_eq!(&response_body[0].display_name, "");
}
