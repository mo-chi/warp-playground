use tracing_subscriber::fmt::format::FmtSpan;

mod controllers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    warp::serve(routes::routes())
        .run(([127, 0, 0, 1], 8000))
        .await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use crate::models::user::Request;
    use crate::routes;

    #[tokio::test]
    async fn test_index() {
        let index = routes::routes();

        let resp = request().method("GET").path("/").reply(&index).await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "{\"message\":\"Hello World\"}");
    }

    #[tokio::test]
    async fn test_users_list() {
        let index = routes::routes();

        let resp = request().method("GET").path("/users").reply(&index).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_users_get() {
        let index = routes::routes();

        let resp = request().method("GET").path("/users/1").reply(&index).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_users_post() {
        let index = routes::routes();

        let resp = request()
            .method("POST")
            .path("/users")
            .json(&body())
            .reply(&index)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_users_put() {
        let index = routes::routes();

        let resp = request()
            .method("PUT")
            .path("/users/1")
            .json(&body())
            .reply(&index)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_users_delete() {
        let index = routes::routes();

        let resp = request()
            .method("DELETE")
            .path("/users/1")
            .reply(&index)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    fn body() -> Request {
        Request {
            name: "alice".to_string(),
        }
    }
}
