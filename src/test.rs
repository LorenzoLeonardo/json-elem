use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use test_case::test_case;

use crate::jsonelem::JsonElem;

#[test]
fn integer() {
    let val = JsonElem::Integer(-1234567890);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "-1234567890");
}

#[test]
fn float() {
    let val = JsonElem::Float(123456.7890);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "123456.789");
}

#[test]
fn bool() {
    let val = JsonElem::Bool(true);
    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), "true");
}

#[test]
fn string() {
    let val = JsonElem::String("the quick brown fox jumps over the lazy dog.".into());
    assert_eq!(
        serde_json::to_string(&val).unwrap().as_str(),
        r#""the quick brown fox jumps over the lazy dog.""#
    );
}

#[test]
fn vec() {
    let val = JsonElem::Vec(vec![
        JsonElem::String("54".into()),
        JsonElem::String("true".into()),
        JsonElem::String("always".into()),
    ]);
    assert_eq!(
        serde_json::to_string(&val).unwrap().as_str(),
        r#"["54","true","always"]"#
    );
}

#[test_case("test key", JsonElem::Bool(true), r#"{"test key":true}"#; "Hashmap with JsonValue::Bool")]
#[test_case("test key", JsonElem::String("test val".to_string()), r#"{"test key":"test val"}"#; "Hashmap with JsonValue::String")]
#[test_case("test key", JsonElem::Integer(1234567890), r#"{"test key":1234567890}"#; "Hashmap with JsonValue::Integer")]
#[test_case("test key", JsonElem::Float(12345.6789), r#"{"test key":12345.6789}"#; "Hashmap with JsonValue::Float")]
#[test_case("test key", JsonElem::Vec(vec![JsonElem::String("test val".to_string()), JsonElem::Bool(true), JsonElem::Integer(123456789), JsonElem::Float(12345.6789)]),
    r#"{"test key":["test val",true,123456789,12345.6789]}"#; "Hashmap with JsonValue::Vec")]
fn hashmap(given_key: &str, given_value: JsonElem, expected: &str) {
    let mut hash = HashMap::new();

    hash.insert(given_key.into(), given_value);
    let val = JsonElem::HashMap(hash);

    assert_eq!(serde_json::to_string(&val).unwrap().as_str(), expected);
}

#[test_case(JsonElem::Bool(true), r#"{"mainkey":{"key1":true}}"#; "bool")]
#[test_case(JsonElem::Integer(123456789), r#"{"mainkey":{"key1":123456789}}"#; "integer")]
#[test_case(JsonElem::String("test val".to_string()), r#"{"mainkey":{"key1":"test val"}}"#; "string")]
#[test_case(JsonElem::Float(12345.6789), r#"{"mainkey":{"key1":12345.6789}}"#; "float")]
#[test_case(JsonElem::Vec(vec![JsonElem::String("test val".to_string()), JsonElem::Bool(true), JsonElem::Integer(123456789), JsonElem::Float(12345.6789)]),
    r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#; "vec")]
fn hashmap_within_a_hashmap(val: JsonElem, expected: &str) {
    let mut hash = HashMap::new();

    hash.insert("key1".into(), val);

    let mut given = HashMap::new();

    given.insert("mainkey", JsonElem::HashMap(hash));

    assert_eq!(serde_json::to_string(&given).unwrap().as_str(), expected);
}

#[test]
fn json_elem_convert_to_any_type_with_deserialize_downed() {
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
        JsonElem::String("process name".to_string()),
    );
    hash.insert(
        "provider".to_string(),
        JsonElem::String("provider name".to_string()),
    );
    hash.insert(
        "authorization_endpoint".to_string(),
        JsonElem::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        ),
    );
    hash.insert(
        "token_endpoint".to_string(),
        JsonElem::String("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()),
    );
    hash.insert(
        "device_auth_endpoint".to_string(),
        JsonElem::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode".to_string(),
        ),
    );
    hash.insert(
        "scopes".to_string(),
        JsonElem::Vec(vec![
            JsonElem::String("offline_access".to_string()),
            JsonElem::String("https://outlook.office.com/SMTP.Send".to_string()),
            JsonElem::String("https://outlook.office.com/User.Read".to_string()),
        ]),
    );
    hash.insert(
        "client_id".to_string(),
        JsonElem::String("client-id-12345".to_string()),
    );
    hash.insert(
        "client_secret".to_string(),
        JsonElem::String("secret-12345".to_string()),
    );

    let given = JsonElem::HashMap(hash);
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
fn json_elem_convert_from_any_type_with_deserialize_downed() {
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
        JsonElem::String("process name".to_string()),
    );
    expected.insert(
        "provider".to_string(),
        JsonElem::String("provider name".to_string()),
    );
    expected.insert(
        "authorization_endpoint".to_string(),
        JsonElem::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
        ),
    );
    expected.insert(
        "token_endpoint".to_string(),
        JsonElem::String("https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string()),
    );
    expected.insert(
        "device_auth_endpoint".to_string(),
        JsonElem::String(
            "https://login.microsoftonline.com/common/oauth2/v2.0/devicecode".to_string(),
        ),
    );
    expected.insert(
        "scopes".to_string(),
        JsonElem::Vec(vec![
            JsonElem::String("offline_access".to_string()),
            JsonElem::String("https://outlook.office.com/SMTP.Send".to_string()),
            JsonElem::String("https://outlook.office.com/User.Read".to_string()),
        ]),
    );
    expected.insert(
        "client_id".to_string(),
        JsonElem::String("client-id-12345".to_string()),
    );
    expected.insert(
        "client_secret".to_string(),
        JsonElem::String("secret-12345".to_string()),
    );

    let result = JsonElem::convert_from(&given).unwrap();

    if let JsonElem::HashMap(result) = result {
        assert_eq!(expected, result);
    } else {
        panic!("Invalid value");
    }
}

#[test]
fn test_from_json_string_to_jsonelem() {
    let given = r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#;
    let result = JsonElem::try_from(given).unwrap();

    let mut expected = HashMap::new();
    let mut hash = HashMap::new();
    hash.insert(
        "key1".into(),
        JsonElem::Vec(vec![
            JsonElem::String("test val".to_string()),
            JsonElem::Bool(true),
            JsonElem::Integer(123456789),
            JsonElem::Float(12345.6789),
        ]),
    );
    expected.insert("mainkey".into(), JsonElem::HashMap(hash));

    println!("{:?}", result);
    assert_eq!(JsonElem::HashMap(expected), result);
}

#[test]
fn test_from_json_slice_to_jsonelem() {
    let given = r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#.as_bytes();
    let result = JsonElem::try_from(given).unwrap();

    let mut expected = HashMap::new();
    let mut hash = HashMap::new();
    hash.insert(
        "key1".into(),
        JsonElem::Vec(vec![
            JsonElem::String("test val".to_string()),
            JsonElem::Bool(true),
            JsonElem::Integer(123456789),
            JsonElem::Float(12345.6789),
        ]),
    );
    expected.insert("mainkey".into(), JsonElem::HashMap(hash));

    assert_eq!(JsonElem::HashMap(expected), result);
}

#[test]
fn test_from_jsonelem_to_json_string() {
    let mut given = HashMap::new();
    let mut hash = HashMap::new();
    hash.insert(
        "key1".into(),
        JsonElem::Vec(vec![
            JsonElem::String("test val".to_string()),
            JsonElem::Bool(true),
            JsonElem::Integer(123456789),
            JsonElem::Float(12345.6789),
        ]),
    );
    given.insert("mainkey".into(), JsonElem::HashMap(hash));

    let given = JsonElem::HashMap(given);
    let expected = r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#;
    let result: String = given.try_into().unwrap();

    assert_eq!(expected, result.as_str());
}

#[test]
fn test_from_jsonelem_to_json_slice() {
    let mut given = HashMap::new();
    let mut hash = HashMap::new();
    hash.insert(
        "key1".into(),
        JsonElem::Vec(vec![
            JsonElem::String("test val".to_string()),
            JsonElem::Bool(true),
            JsonElem::Integer(123456789),
            JsonElem::Float(12345.6789),
        ]),
    );
    given.insert("mainkey".into(), JsonElem::HashMap(hash));

    let given = JsonElem::HashMap(given);
    let expected = r#"{"mainkey":{"key1":["test val",true,123456789,12345.6789]}}"#.as_bytes();
    let result: Vec<u8> = given.try_into().unwrap();
    assert_eq!(expected, result.as_slice());
}
