use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use test_case::test_case;

use crate::json_value::JsonValue;

#[test]
fn integer() {
    let val = JsonValue::Integer(-1234567890);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "-1234567890");
}

#[test]
fn float() {
    let val = JsonValue::Float(123456.7890);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "123456.789");
}

#[test]
fn bool() {
    let val = JsonValue::Bool(true);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "true");
}

#[test]
fn string() {
    let val = JsonValue::String("the quick brown fox jumps over the lazy dog.".into());
    assert_eq!(
        serde_json::to_string(&val).unwrap().as_str(),
        r#""the quick brown fox jumps over the lazy dog.""#
    );
}

#[test]
fn vec() {
    let val = JsonValue::Vec(vec![
        JsonValue::String("54".into()),
        JsonValue::String("true".into()),
        JsonValue::String("always".into()),
    ]);
    assert_eq!(
        serde_json::to_string(&val).unwrap().as_str(),
        r#"["54","true","always"]"#
    );
}

#[test_case("test key", JsonValue::Bool(true), r#"{"test key":true}"#; "Hashmap with JsonValue::Bool")]
#[test_case("test key", JsonValue::String("test val".to_string()), r#"{"test key":"test val"}"#; "Hashmap with JsonValue::String")]
#[test_case("test key", JsonValue::Integer(1234567890), r#"{"test key":1234567890}"#; "Hashmap with JsonValue::Integer")]
#[test_case("test key", JsonValue::Float(12345.6789), r#"{"test key":12345.6789}"#; "Hashmap with JsonValue::Float")]
#[test_case("test key", JsonValue::Vec(vec![JsonValue::String("test val".to_string()), JsonValue::Bool(true), JsonValue::Integer(123456789), JsonValue::Float(12345.6789)]),
    r#"{"test key":["test val",true,123456789,12345.6789]}"#; "Hashmap with JsonValue::Vec")]
fn hashmap(given_key: &str, given_value: JsonValue, expected: &str) {
    let mut hash = HashMap::new();

    hash.insert(given_key.into(), given_value);
    let val = JsonValue::HashMap(hash);

    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), expected);
}

#[test_case(JsonValue::Bool(true), r#"{"mainkey":{"key1":true}}"#; "bool")]
#[test_case(JsonValue::Integer(123456789), r#"{"mainkey":{"key1":123456789}}"#; "integer")]
#[test_case(JsonValue::String("test val".to_string()), r#"{"mainkey":{"key1":"test val"}}"#; "string")]
#[test_case(JsonValue::Float(12345.6789), r#"{"mainkey":{"key1":12345.6789}}"#; "float")]
#[test_case(JsonValue::Vec(vec![JsonValue::String("test val".to_string()), JsonValue::Bool(true), JsonValue::Integer(123456789), JsonValue::Float(12345.6789)]),
    r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#; "vec")]
fn hashmap_within_a_hashmap(val: JsonValue, expected: &str) {
    let mut hash = HashMap::new();

    hash.insert("key1".into(), val);

    let mut given = HashMap::new();

    given.insert("mainkey", JsonValue::HashMap(hash));

    assert_eq!(serde_json::to_string(&given).unwrap().as_str(), expected);
}

#[test]
fn json_value_convert_to_any_type_with_deserialize_downed() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Provider {
        process: String,
        provider: String,
        authorization_endpoint: String,
        token_endpoint: String,
        device_auth_endpoint: String,
        scopes: Vec<String>,
        client_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    }
    let mut hash = HashMap::new();

    hash.insert(
        "process".to_string(),
        JsonValue::String("process name".to_string()),
    );
    hash.insert(
        "provider".to_string(),
        JsonValue::String("provider name".to_string()),
    );
    hash.insert(
        "authorization_endpoint".to_string(),
        JsonValue::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        ),
    );
    hash.insert(
        "token_endpoint".to_string(),
        JsonValue::String("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()),
    );
    hash.insert(
        "device_auth_endpoint".to_string(),
        JsonValue::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode".to_string(),
        ),
    );
    hash.insert(
        "scopes".to_string(),
        JsonValue::Vec(vec![
            JsonValue::String("offline_access".to_string()),
            JsonValue::String("https://outlook.office.com/SMTP.Send".to_string()),
            JsonValue::String("https://outlook.office.com/User.Read".to_string()),
        ]),
    );
    hash.insert(
        "client_id".to_string(),
        JsonValue::String("client-id-12345".to_string()),
    );
    hash.insert(
        "client_secret".to_string(),
        JsonValue::String("secret-12345".to_string()),
    );

    let given = JsonValue::HashMap(hash);
    let result = given.convert_to::<Provider>().unwrap();

    let expected = Provider {
        process: String::from("process name"),
        provider: String::from("provider name"),
        authorization_endpoint: String::from(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        ),
        token_endpoint: String::from("https://login.microsoftonline.com/common/oauth2/v2.0/token"),
        device_auth_endpoint: String::from(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode",
        ),
        scopes: vec![
            String::from("offline_access"),
            String::from("https://outlook.office.com/SMTP.Send"),
            String::from("https://outlook.office.com/User.Read"),
        ],
        client_id: String::from("client-id-12345"),
        client_secret: Some(String::from("secret-12345")),
    };

    assert_eq!(expected, result);
}

#[test]
fn json_value_convert_from_any_type_with_deserialize_downed() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Provider {
        process: String,
        provider: String,
        authorization_endpoint: String,
        token_endpoint: String,
        device_auth_endpoint: String,
        scopes: Vec<String>,
        client_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_secret: Option<String>,
    }

    let given = Provider {
        process: String::from("process name"),
        provider: String::from("provider name"),
        authorization_endpoint: String::from(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        ),
        token_endpoint: String::from("https://login.microsoftonline.com/common/oauth2/v2.0/token"),
        device_auth_endpoint: String::from(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode",
        ),
        scopes: vec![
            String::from("offline_access"),
            String::from("https://outlook.office.com/SMTP.Send"),
            String::from("https://outlook.office.com/User.Read"),
        ],
        client_id: String::from("client-id-12345"),
        client_secret: Some(String::from("secret-12345")),
    };

    let mut expected = HashMap::new();

    expected.insert(
        "process".to_string(),
        JsonValue::String("process name".to_string()),
    );
    expected.insert(
        "provider".to_string(),
        JsonValue::String("provider name".to_string()),
    );
    expected.insert(
        "authorization_endpoint".to_string(),
        JsonValue::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        ),
    );
    expected.insert(
        "token_endpoint".to_string(),
        JsonValue::String("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()),
    );
    expected.insert(
        "device_auth_endpoint".to_string(),
        JsonValue::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode".to_string(),
        ),
    );
    expected.insert(
        "scopes".to_string(),
        JsonValue::Vec(vec![
            JsonValue::String("offline_access".to_string()),
            JsonValue::String("https://outlook.office.com/SMTP.Send".to_string()),
            JsonValue::String("https://outlook.office.com/User.Read".to_string()),
        ]),
    );
    expected.insert(
        "client_id".to_string(),
        JsonValue::String("client-id-12345".to_string()),
    );
    expected.insert(
        "client_secret".to_string(),
        JsonValue::String("secret-12345".to_string()),
    );

    let result = JsonValue::convert_from(&given).unwrap();

    if let JsonValue::HashMap(result) = result {
        assert_eq!(expected, result);
    } else {
        panic!("Invalid value");
    }
}
