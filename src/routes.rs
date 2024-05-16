use super::controllers;
use super::models;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let index = index();

    index.or(users::routes())
}

/// GET /
fn index() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(controllers::index)
        .with(warp::trace::named("index"))
}

mod users {
    use super::controllers::users;
    use super::models::user::Request;
    use warp::Filter;

    pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        get_users()
            .or(get_user())
            .or(create_user())
            .or(update_user())
            .or(delete_user())
            .with(warp::trace::named("users"))
    }

    /// GET /users
    fn get_users() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("users").and(warp::get()).and_then(users::list)
    }

    /// GET /users/:id
    fn get_user() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("users" / u64)
            .and(warp::get())
            .and_then(users::get)
    }

    /// POST /users with JSON body
    fn create_user() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!("users")
            .and(warp::post())
            .and(json_body())
            .and_then(users::create)
    }

    /// PUT /users/:id with JSON body
    fn update_user() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!("users" / u64)
            .and(warp::put())
            .and(json_body())
            .and_then(users::update)
    }

    /// DELETE /users/:id
    fn delete_user() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
    {
        warp::path!("users" / u64)
            .and(warp::delete())
            .and_then(users::delete)
    }

    fn json_body() -> impl Filter<Extract = (Request,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
