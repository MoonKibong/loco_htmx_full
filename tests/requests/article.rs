use loco_htmx_full::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_echo() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let payload = serde_json::json!({
            "foo": "bar",
        });

        let res = request.post("/article/echo").json(&payload).await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), serde_json::to_string(&payload).unwrap());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_request_root() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/article").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "hello");
    })
    .await;
}
