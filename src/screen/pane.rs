
use super::{Component, CResult};

pub trait Pane: Component {
    fn tile(&self) -> CResult<()>;
}
