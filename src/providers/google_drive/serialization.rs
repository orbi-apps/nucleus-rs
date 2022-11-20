// use std::{collections::HashMap, future::Future};

// use google_drive3::oauth2::storage::TokenInfo;
// use serde::{Serialize, Deserialize, ser::SerializeStruct, de::Visitor};

// use super::GoogleDrive;

// impl Serialize for GoogleDrive {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         let mut state = serializer.serialize_struct("GoogleDrive", 1)?;
//         let tokens = (*self.tokens.as_ref().lock().unwrap()).clone();
//         state.serialize_field("tokens", &tokens)?;
//         state.serialize_field("client_secret", &self.client_secret)?;
//         state.end()
//     }
// }

// struct TokenVisitor {}

// impl<'de> Visitor<'de> for TokenVisitor {
//     type Value = (String, HashMap<String, TokenInfo>);

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("Could not deserialize tokens")
//     }

//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//         where
//             A: serde::de::MapAccess<'de>, {
//         let mut values = (None, None);
//         while let Some(key) = map.next_key::<String>()? {
//             if key == "client_secret".to_string() {
//                 values.0 = Some(map.next_value::<String>()?);
//             } else if key == "tokens".to_string() {
//                 values.1 = Some(map.next_value::<HashMap<String, TokenInfo>>()?);
//             }
//         }
//         Ok((values.0.unwrap(), values.1.unwrap()))
//     }
// }

// impl<'de> Deserialize<'de> for GoogleDrive {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//             let values = deserializer.deserialize_map(TokenVisitor {}).unwrap();
//             let google_drive_future = GoogleDrive::new(values.0, values.1);
//             let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

//             let x = rt.block_on(google_drive_future).unwrap();
//             Ok(x)
//     }
// }