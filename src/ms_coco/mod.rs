pub mod protos;

use crate::error;
use protobuf;
use std;
use std::collections;
use std::fmt;
use std::result;

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

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_label_name(&self, id: &i32) -> Result<String, LabelNotFound> {
        Ok(self
            .data
            .get(id)
            .ok_or_else(|| LabelNotFound { id: id.clone() })?
            .to_string())
    }
}
