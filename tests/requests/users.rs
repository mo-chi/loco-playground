use insta::{assert_debug_snapshot, with_settings};
use loco_playground::app::App;
use loco_rs::testing;
use serial_test::serial;

use super::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("users_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_get_users() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let test_user = prepare_data::user_login(&request).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&test_user.token);
        let users = request
            .get("/api/users")
            .add_header(auth_key, auth_value)
            .await;

        with_settings!({
            filters => {
                let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                combined_filters.extend(vec![(r#"\"id\\":\d+"#, r#""id\":ID"#)]);
                combined_filters
            }
        }, {
            assert_debug_snapshot!(
                (users.status_code(), users.text())
            );
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_user() {
    configure_insta!();

    testing::request::<App, _, _>(|request, ctx| async move {
        testing::seed::<App>(&ctx.db).await.unwrap();

        let test_user = prepare_data::user_login(&request).await;

        let (auth_key, auth_value) = prepare_data::auth_header(&test_user.token);
        let users = request
            .get("/api/users/1")
            .add_header(auth_key, auth_value)
            .await;

        with_settings!({
            filters => {
                let mut combined_filters = testing::CLEANUP_DATE.to_vec();
                combined_filters.extend(vec![(r#"\"id\\":\d+"#,r#""id\":ID"#)]);
                combined_filters
            }
        }, {
            assert_debug_snapshot!(
                (users.status_code(), users.text())
            )
        });
    })
    .await;
}
