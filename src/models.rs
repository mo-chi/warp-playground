pub mod index {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Response {
        pub message: String,
    }
}

pub mod user {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Request {
        pub name: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Response {
        pub id: u64,
        pub name: String,
    }
}
