use dotenv::dotenv;
use kalshi::Kalshi;
use std::env;
use std::thread;
use std::time::Duration;

extern crate kalshi;

enum APIType {
    Live,
    Demo,
}

fn retreive_credentials(setting: APIType, username: &mut String, pass: &mut String) -> () {
    match setting {
        APIType::Live => {
            if let Ok(key) = env::var("LIVE_PASSWORD") {
                println!("got password");
                *pass = key;
            }
            if let Ok(user) = env::var("LIVE_USER_NAME") {
                println!("got user");
                *username = user;
            }
        }

        APIType::Demo => {
            if let Ok(key) = env::var("DEMO_PASSWORD") {
                println!("got password");
                *pass = key;
            }
            if let Ok(user) = env::var("DEMO_USER_NAME") {
                println!("got user");
                *username = user;
            }
        }
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut username = "dummy".to_string();
    let mut password = "dummy".to_string();
    retreive_credentials(APIType::Demo, &mut username, &mut password);

    // main testing logic, ignoring unit tests for now
    let mut kalshi_instance = Kalshi::new();

    kalshi_instance.build_base_url(kalshi::TradingEnvironment::DemoMode);

    kalshi_instance.login(&username, &password).await;

    let token = kalshi_instance.get_user_token().unwrap();
    println!("{}", token);
    let balance = kalshi_instance.get_balance().await.unwrap();
    println!("{}", balance);
    let my_ticker = String::from("GOVSHUTLENGTH-23DEC31-T14");
    //let my_events = kalshi_instance.get_multiple_events(Some(1), Some("CgYIgLDxhgcSEUFNQVpPTkZUQy0yOURFQzMx".to_string()), None, None, None).await.unwrap();
    //let my_orders = kalshi_instance.get_multiple_orders(None, None, None, None, None, Some(1), None).await.unwrap();
    /*let my_settlements = kalshi_instance
        .get_portfolio_settlements(None, None)
        .await
        .unwrap();
    */
    //let my_positions = kalshi_instance.get_user_positions(None, None, None, None, None).await.unwrap();
    //println!("{:?}", my_events);
    let bought_order = kalshi_instance.create_order("buy".to_string(), None, 1, "yes".to_string(), my_ticker.to_string(), "limit".to_string(), None, None, None, None, Some(1)).await.unwrap();
    let curr_id = bought_order.order_id.clone();
    println!("{}", curr_id);
    //thread::sleep(Duration::from_secs(1));
    let cancelled_order = kalshi_instance.get_single_order(&curr_id).await.unwrap();
    println!("{:?}", cancelled_order);
}
