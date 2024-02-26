use hyper::header::{HeaderMap, HeaderValue, AUTHORIZATION};
pub async fn jwt_middleware(req: axum::http::Request<B>, next: axum::http::BoxHandler<B>,) -> Result<axum::http::Response<hyper::Body>, axum::BoxError>
    where B: axum::body::HttpBody + Default + Send + Sync + 'static,
{
    if let Some(authorization) = req.headers().get(AUTHORIZATION) {
        if authorization == HeaderValue::from_static("Bearer my_token") {
            return next.call(req).await;
        }
    }
    // 验证失败，返回 401 Unauthorized 响应
    Ok(axum::http::Response::builder()
        .status(401)
        .body(hyper::Body::empty())
        .unwrap())
}