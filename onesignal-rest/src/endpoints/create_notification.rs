// [Ref](https://documentation.onesignal.com/reference/create-notification)

use core::ops::Deref;
use std::collections::HashMap;

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::ResponseBodyErrJson,
    types::language::Language,
};

pub const URL: &str = "https://onesignal.com/api/v1/notifications";

//
#[derive(Debug, Clone)]
pub struct CreateNotification {
    pub req_body_json: CreateNotificationRequestBodyJson,
    pub rest_api_key: Box<str>,
}

impl CreateNotification {
    pub fn new(
        req_body_json: CreateNotificationRequestBodyJson,
        rest_api_key: impl AsRef<str>,
    ) -> Self {
        Self {
            req_body_json,
            rest_api_key: rest_api_key.as_ref().into(),
        }
    }
}

impl Endpoint for CreateNotification {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<CreateNotificationResponseBodyOkJson>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = Url::parse(URL).map_err(EndpointError::MakeRequestUrlFailed)?;

        let body = serde_json::to_vec(&self.req_body_json)
            .map_err(EndpointError::SerRequestBodyJsonFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url.as_str())
            .header(USER_AGENT, "onesignal-rest")
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .header(CONTENT_TYPE, MIME_APPLICATION_JSON)
            .header(AUTHORIZATION, format!("Basic {}", self.rest_api_key))
            .body(body)
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
            )),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
#[derive(Serialize, Debug, Clone, Default)]
pub struct CreateNotificationRequestBodyJson {
    pub app_id: Box<str>,
    //
    // Send to Specific Devices https://documentation.onesignal.com/reference/create-notification#send-to-specific-devices
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_player_ids: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_user_ids: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_email_tokens: Option<Vec<Box<str>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_phone_numbers: Option<Vec<Box<str>>>,
    //
    // X Channel Properties
    //
    pub contents: HashMap<Language, Box<str>>,
    //
    // Push Channel Properties https://documentation.onesignal.com/reference/push-channel-properties
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Map<String, Value>>,
}

impl CreateNotificationRequestBodyJson {
    pub fn new(app_id: impl AsRef<str>, contents: HashMap<Language, Box<str>>) -> Self {
        Self {
            app_id: app_id.as_ref().into(),
            contents,
            ..Default::default()
        }
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateNotificationResponseBodyOkJson {
    pub id: Box<str>,
    pub recipients: usize,

    pub errors: Option<CreateNotificationResponseBodyOkJsonErrors>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateNotificationResponseBodyOkJsonErrors(Value);

impl Deref for CreateNotificationResponseBodyOkJsonErrors {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CreateNotificationResponseBodyOkJsonErrors {
    /*
    when "Player ID" Subscribed is "Ios Simulator Unsupported"
    {"invalid_player_ids": ["47f5a392-bbc0-47bb-9b49-bec50265fc77"]}
    */
    pub fn is_invalid_player_ids(&self) -> bool {
        self.as_object()
            .map(|x| x.get("invalid_player_ids").is_some())
            == Some(true)
    }

    pub fn is_invalid_external_user_ids(&self) -> bool {
        self.as_object()
            .map(|x| x.get("invalid_external_user_ids").is_some())
            == Some(true)
    }

    pub fn is_invalid_phone_numbers(&self) -> bool {
        self.as_object()
            .map(|x| x.get("invalid_phone_numbers").is_some())
            == Some(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body_ok_json() {
        //
        match serde_json::from_str::<CreateNotificationResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.id, "b98881cc-1e94-4366-bbd9-db8f3429292b".into());
                assert!(ok_json.errors.is_none());
            }
            Err(err) => panic!("{}", err),
        }

        //
        match serde_json::from_str::<CreateNotificationResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__ok__invalid_external_user_ids.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.errors.as_ref().unwrap().is_invalid_external_user_ids());
            }
            Err(err) => panic!("{}", err),
        }

        //
        match serde_json::from_str::<CreateNotificationResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__ok__invalid_player_ids.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.errors.as_ref().unwrap().is_invalid_player_ids());
                assert_eq!(
                    ok_json.errors.unwrap().get("invalid_player_ids").unwrap(),
                    &Value::from(vec!["b186912c-cf25-4688-8218-06cb13e09a4f"])
                )
            }
            Err(err) => panic!("{}", err),
        }

        //
        match serde_json::from_str::<CreateNotificationResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__ok__invalid_phone_numbers.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.errors.as_ref().unwrap().is_invalid_phone_numbers());
            }
            Err(err) => panic!("{}", err),
        }

        //
        match serde_json::from_str::<CreateNotificationResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__ok__no_subscribed_devices.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.errors.is_some());
            }
            Err(err) => panic!("{}", err),
        }
    }
}
