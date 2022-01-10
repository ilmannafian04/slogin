use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Response<T>
where
    T: Serialize + Clone,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub struct ResponseBuilder<T>
where
    T: Serialize + Clone,
{
    response: Response<T>,
}

impl<T> ResponseBuilder<T>
where
    T: Serialize + Clone,
{
    pub fn new() -> Self {
        Self {
            response: Response {
                message: None,
                error: None,
            },
        }
    }

    pub fn error<'a>(&'a mut self, error: String) -> &'a mut Self {
        self.response.error = Some(error);
        self
    }

    pub fn message<'a>(&'a mut self, message: T) -> &'a mut Self {
        self.response.message = Some(message);
        self
    }

    pub fn build(&self) -> Response<T> {
        self.response.clone()
    }
}

#[derive(Deserialize)]
pub struct StoreBody {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct RetrieveQuery {
    pub key: String,
}
