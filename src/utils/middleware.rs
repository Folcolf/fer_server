use axum::http::StatusCode;
use axum::{
    body::{Body, Bytes},
    http::{Request, Response},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde_json::json;

/// Create response for not found
pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

/// A middleware that log the request and response and transform the response to a JSON
pub async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let method = req.method().to_string();
    let uri = req.uri().to_string();

    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res_bytes = bytes.clone();
    let res = Response::from_parts(parts, Body::from(bytes));

    match res.status() {
        StatusCode::OK | StatusCode::CREATED => {
            tracing::info!("{} {} {}", method, uri, res.status())
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            tracing::error!("{} {} {} {:?}", method, uri, res.status(), res.body())
        }
        _ => tracing::warn!("{} {} {}", method, uri, res.status()),
    }
    Ok(format_response(res, res_bytes).await)
}

/// Buffer the body and print it to stdout.
async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
/// Format the response body to be a JSON object
async fn format_response<B>(res: Response<B>, bytes: Bytes) -> impl IntoResponse {
    // tracing::error!("404 Not Found");
    let json = serde_json::from_slice::<serde_json::Value>(&bytes);

    Json(json!({
      "status": res.status().as_str(),
      "timestamp": current_date(),
      "body": json.unwrap_or(json!({})),
    }))
}

fn current_date() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}
