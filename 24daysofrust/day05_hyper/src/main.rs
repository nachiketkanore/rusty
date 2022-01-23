use std::collections::HashMap;

async fn get_req() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("My IP address = {:#?}", resp);
    Ok(())
}

async fn get_req2() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://ifconfig.me/all")
        .await?
        .text()
        .await?;
    println!("Response = {}", body);
    Ok(())
}

async fn post_req() -> Result<(), Box<dyn std::error::Error>> {
    // default accepted format = List of pairs of strings
    let params = [("first_name", "nachiket"), ("last_name", "kanore")];
    // let params = ["nachiket", "kanore"];
    let client = reqwest::Client::new();
    let res = client
        .post("https://httpbin.org/post")
        .form(&params)
        .send()
        .await?;
    println!("Response = {:?}", res);
    Ok(())
}

async fn post_req_json() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();

    map.insert("name", "nachiket");
    map.insert("college", "pict");
    map.insert("city", "pune");
    map.insert("home", "ahmednagar");

    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    // was not showing `data` attribute of response
    // so had to convert it to text
    // still shows RAW format
    println!("Response of POST request = {:?}", response.text().await?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_req().await?;
    get_req2().await?;
    post_req().await?;
    post_req_json().await?;
    Ok(())
}
