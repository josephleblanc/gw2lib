use crate::items::{AttributeType, BulkEndpoint, ItemId};
use crate::{Endpoint, EndpointWithId};
use serde::{Deserialize, Serialize};

pub type StatsId = u32;

// I used the name `Attribute` here, though it is also used for a different
// struct in items.rs, which does not include the `multiplier` field. It is
// easier to just call this struct `Attribute` because it matches the API, but
// it could be confusing to have these two structs with the same name.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    attribute: AttributeType,
    multiplier: f32,
    value: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ItemStats {
    id: ItemId,
    name: String,
    attributes: Vec<Attribute>,
}

impl Endpoint for ItemStats {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/itemstats";
    const VERSION: &'static str = "2022-07-22T00:00:00.000Z";
}

impl EndpointWithId for ItemStats {
    type IdType = StatsId;
}

impl BulkEndpoint for ItemStats {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
