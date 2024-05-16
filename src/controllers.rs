use super::models::{index, user};
use std::convert::Infallible;

pub fn index() -> Result<impl warp::Reply, Infallible> {
    let response = index::Response {
        message: "Hello World".to_string(),
    };

    Ok(warp::reply::json(&response))
}

pub mod users {
    use super::user::{Request, Response};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn list() -> Result<impl warp::Reply, Infallible> {
        let users = vec![
            Response {
                id: 1,
                name: "alice".to_string(),
            },
            Response {
                id: 2,
                name: "bob".to_string(),
            },
        ];

        Ok(warp::reply::json(&users))
    }

    pub async fn get(_id: u64) -> Result<impl warp::Reply, Infallible> {
        let user: Response = Response {
            id: 1,
            name: "alice".to_string(),
        };

        Ok(warp::reply::json(&user))
    }

    pub async fn create(_user: Request) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::CREATED)
    }

    pub async fn update(_id: u64, _user: Request) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }

    pub async fn delete(_id: u64) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }
}
