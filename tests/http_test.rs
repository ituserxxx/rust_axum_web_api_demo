use reqwest::StatusCode;
use reqwest::Client;
const URL: &str = "http://127.0.0.1:8001/";

//详细 test 请求加参数 RUST_BACKTRACE=full cargo test --test http_test user_list
//详细 test 请求加参数 RUST_BACKTRACE=1 cargo test --test http_test user_list

// cargo test --test http_test root
#[tokio::test]
async fn root() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let response = client.get(format!("{}", URL)).send().await?;
    assert_eq!(response.status(), StatusCode::OK);
    // 根据需要处理响应的内容
    let body = response.text().await?;
    println!("Response body: {}", body); // 在这里打印响应内容
    Ok(())
}


// cargo test --test http_test user_list
#[tokio::test]
async fn user_list() {
    let client = Client::new();
    let body = ""; // POST 请求的 body 数据
    let response = client.post(format!("{}{}", URL,"user/list"))
        .body(body)
        .send()
        .await
        .expect("Failed to send POST request");
    let status_code = response.status();
    println!("Response status code: {}", status_code);

    let response_body = response.text().await.expect("Failed to read response body");
    println!("Response body: {}", response_body); // 打印响应内容

    assert!(status_code.is_success(), "Request failed with status code: {}", status_code);
}
