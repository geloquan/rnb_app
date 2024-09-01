use serde::{Serialize, Deserialize};
use yew::{Properties, Reducible};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct Focus {
    pub stroke: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties)]
pub struct Unfocus {
    pub stroke: String,
    pub fill: String,
}