use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use entity::user::Model;
use serde::{Deserialize, Serialize};

// For List response.
#[derive(Serialize)]
pub struct ListResponse<T>
where
    T: Serialize,
{
    pub code: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    pub data: T,
}

impl<T> ListResponse<T>
where
    T: Serialize,
{
    pub fn new(page: Option<u64>, total: Option<u64>, data: T, code: usize) -> ListResponse<T>
    where
        T: Sized,
    {
        Self {
            code,
            page,
            total,
            data,
        }
    }
}

impl<T> Responder for ListResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::OK)
            .body(body)
    }
}

#[derive(Serialize)]
pub struct RetrieveResponse<T: Serialize> {
    pub code: usize,
    pub data: T,
}

impl<T> RetrieveResponse<T>
where
    T: Serialize,
{
    pub fn new(code: usize, data: T) -> RetrieveResponse<T> {
        RetrieveResponse { code, data }
    }
}

impl<T> Responder for RetrieveResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::OK)
            .body(body)
    }
}

#[derive(Serialize)]
pub struct CreatedResponse<T: Serialize> {
    pub code: usize,
    pub data: T,
}

impl<T> CreatedResponse<T>
where
    T: Serialize,
{
    pub fn new(code: usize, data: T) -> CreatedResponse<T> {
        CreatedResponse { code, data }
    }
}

impl<T> Responder for CreatedResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::CREATED)
            .body(body)
    }
}

#[derive(Serialize)]
pub struct DeletedResponse;

impl Responder for DeletedResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::OK)
            .body(r#"" ""#)
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    code: usize,
    access_token: String,
    data: Model,
}

impl LoginResponse {
    pub fn new(code: usize, access_token: String, data: Model) -> Self {
        Self {
            code,
            access_token,
            data,
        }
    }
}

impl Responder for LoginResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::MOVED_PERMANENTLY)
            .body(body)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::parse_url;

    #[test]
    fn url_parse() {
        let urls: Vec<&str> = vec![
            "http://localhost:8082/app/users/1/",
            "http://localhost:8082/app/users/1",
            "http://localhost:8082/app/users/",
            "http://localhost:8082/app/users/dddd",
        ];
        let mut res = vec![];
        for url in urls {
            res.push(parse_url(url))
        }
        assert_eq!([true, true, false, false].to_vec(), res)
    }
}

/// distinguish the method of retrieve `GET`  from `Get` that have  similarly `URI`
/// e.g. `Get http:localhost:8080/app/users/`
///e.g. `GET http:localhost:8080/app/users/1`
/// return `true`, if it is `Get` method.
pub fn parse_url(url: &str) -> bool {
    if url.ends_with('/') {
        let urls: Vec<&str> = url.rsplitn(3, '/').collect();
        match urls[1].parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    } else {
        let urls: Vec<&str> = url.rsplitn(2, '/').collect();
        match urls[0].parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

pub const DEFAULT_PER_PAGE: u64 = 5;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}
