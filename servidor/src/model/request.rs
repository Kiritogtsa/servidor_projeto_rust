use crate::NewTrait;

use super::{body::Body, head::Head};
use serde::de::DeserializeOwned;
pub struct Request {
    pub head: Head,
    pub body: Body,
}
