use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrJson {
    pub errors: Vec<Box<str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        //
        match serde_json::from_str::<ErrJson>(include_str!(
            "../../tests/response_body_json_files/err__app_id_not_found.json"
        )) {
            Ok(err_json) => {
                println!("{:?}", err_json);
            }
            Err(err) => panic!("{}", err),
        }

        //
        match serde_json::from_str::<ErrJson>(include_str!(
            "../../tests/response_body_json_files/create_notification__err__missing_contents_en.json"
        )) {
            Ok(err_json) => {
                println!("{:?}", err_json);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
