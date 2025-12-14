use crate::helpers::spawn_app;
use newsletter_api::models::UserProfileAPI;

#[tokio::test]
async fn unauthenticated_user_cannot_retrieve_user_profile() {
    let app = spawn_app().await;
    let user_response = app.get_admin_user().await;

    assert_eq!(401, user_response.status().as_u16());
}

#[tokio::test]
async fn authenticated_user_can_retrieve_user_profile() {
    let app = spawn_app().await;
    app.test_user.login(&app).await;

    let user_response = app.get_admin_user().await;
    assert_eq!(200, user_response.status().as_u16());

    let response_body: UserProfileAPI = user_response.json().await.unwrap();
    assert_eq!(app.test_user.username, response_body.username);
}
