#[tokio::test]
async fn integration_you_must_be_logged_in_to_access_the_admin_dashboard() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let response = app.get_admin_dashboard().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn integration_logout_clears_session_state() {
    let app = crate::tests::integration_tests::api::helpers::spawn_app().await;
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(
        &response,
        "/admin/dashboard",
    );
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
    let response = app.post_logout().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));
    let response = app.get_admin_dashboard().await;
    crate::tests::integration_tests::api::helpers::assert_is_redirect_to(&response, "/login");
}
