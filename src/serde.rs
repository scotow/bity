use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IntOrString {
    Int(u64),
    String(String),
}

#[macro_export]
macro_rules! impl_serde {
    () => {
        pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
            serializer.serialize_str(&format(*value))
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error> where D: serde::Deserializer<'de> {
            Ok(match <crate::serde::IntOrString as serde::Deserialize>::deserialize(deserializer)? {
                crate::serde::IntOrString::Int(n) => n,
                crate::serde::IntOrString::String(s) => parse(&s).map_err(|err| <D::Error as serde::de::Error>::custom(err))?
            })
        }
    };
}