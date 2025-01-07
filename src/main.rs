use reqwest;
use reqwest::header::{CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Slip
{
    id: u32,
    advice: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse
{
    slip: Slip,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.adviceslip.com/advice")
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await.unwrap();
    match response.status()
    {
        reqwest::StatusCode::OK => 
        {
            //things are okay
            
            match response.json::<APIResponse>().await
            {
                Ok(parsed) => println!("{:?}", parsed.slip.advice),
                Err(error_in_question) => println!("{:?}", error_in_question),
            }
        }
        other =>
        {
            panic!("Something went wrong! {:?}", other);
        }
    }
}
