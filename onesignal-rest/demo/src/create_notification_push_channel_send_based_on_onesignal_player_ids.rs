// https://documentation.onesignal.com/reference/push-channel-properties#send-based-on-onesignal-player-ids

/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p onesignal-rest-demo --bin create_notification_push_channel_send_based_on_onesignal_player_ids -- 'YOUR_APP_ID' 'YOUR_REST_API_KEY' 'YOUR_PLAYER_ID'
*/

use std::{collections::HashMap, env, error};

use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use onesignal_rest::{
    endpoints::{common::EndpointRet, CreateNotification, CreateNotificationRequestBodyJson},
    types::Language,
};
use serde_json::Map;

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let app_id = env::args().nth(1).unwrap();
    let rest_api_key = env::args().nth(2).unwrap();
    let player_id = env::args().nth(3).unwrap();

    let client = IsahcClient::new()?;

    let mut create_notification_req_body_json_contents = HashMap::new();
    create_notification_req_body_json_contents.insert(Language::En, "English Message".into());
    let mut create_notification_req_body_json =
        CreateNotificationRequestBodyJson::new(app_id, create_notification_req_body_json_contents);
    create_notification_req_body_json.data = {
        let mut data = Map::new();
        data.insert("abc".into(), 123_usize.into());
        data.insert("foo".into(), "bar".into());
        data.insert("event_performed".into(), true.into());
        Some(data)
    };
    create_notification_req_body_json.include_player_ids = Some(vec![player_id.into()]);
    let create_notification =
        CreateNotification::new(create_notification_req_body_json, rest_api_key);

    let ret = client.respond_endpoint(&create_notification).await?;

    match &ret {
        EndpointRet::Ok(ok_json) => {
            println!("{:?}", ok_json);
        }
        EndpointRet::Other(_) => {
            println!("{:?}", ret);
        }
    }

    Ok(())
}
