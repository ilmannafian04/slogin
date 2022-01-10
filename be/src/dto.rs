use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T>
where
    T: Serialize,
{
    pub message: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
