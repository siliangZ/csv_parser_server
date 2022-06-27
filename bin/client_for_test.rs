use csv_parse_server::CityRecord;

extern crate reqwest;
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1, "missing input csv");
    let csv_path = args[1].clone();
    let content = std::fs::read_to_string(csv_path).unwrap();
    let request_url = "http://127.0.0.1:3030/upload";
    println!("requesting {}", request_url);

    // create multiple request
    let mut handles = vec![];
    for _ in 0..10 {
        let request_url_clone = request_url.clone();
        let content_clone = content.clone();
        let handle = tokio::spawn(async move {
            let client = reqwest::ClientBuilder::new().build().unwrap();
            let response = client
                .post(request_url_clone)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(content_clone)
                .send()
                .await
                .unwrap();
            match response.error_for_status() {
                Ok(res) => {
                    let json_objects = res.json::<Vec<CityRecord>>().await.unwrap();
                    println!("{:?}", json_objects);
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}
