pub mod protos;

use crate::error;
use std::collections;
use std::fmt;
use std::result;

#[derive(PartialEq)]
pub struct LabelNotFound {
    id: i32,
}

impl std::error::Error for LabelNotFound {}

impl fmt::Display for LabelNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Label for ID {} not found", self.id)
    }
}

impl fmt::Debug for LabelNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LabelNotFound {{ id: {}}}", self.id)
    }
}

pub struct LabelMap {
    data: collections::HashMap<i32, String>,
}

impl LabelMap {
    pub fn load() -> result::Result<Self, error::Error> {
        let raw_data = include_str!("mscoco_label_map.pbtxt");
        let data: protos::labelmap::StringIntLabelMapProto =
            protobuf::text_format::parse_from_str(raw_data)?;
        Ok(LabelMap {
            data: data
                .item
                .iter()
                .map(|item| (item.get_id(), item.get_display_name().to_string()))
                .collect::<collections::HashMap<_, _>>(),
        })
    }

    pub fn get_label_name(&self, id: &i32) -> Result<String, LabelNotFound> {
        match self.data.get(id) {
            Some(value) => Ok(value.to_string()),
            None => Err(LabelNotFound { id: *id }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_label_name() {
        let label_map = LabelMap::load().expect("Failed to initialize a Label Map");
        let item = label_map
            .get_label_name(&17)
            .expect("Failed to get an item from the Label Map");
        assert_eq!(item, "cat".to_string());
    }

    #[test]
    fn not_found_label_name() {
        let label_map = LabelMap::load().expect("Failed to initialize a Label Map");
        assert_eq!(
            label_map.get_label_name(&123456789),
            Err(LabelNotFound { id: 123456789 })
        )
    }
}
