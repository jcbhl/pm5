use std::lazy::SyncLazy;
use uuid::Uuid;

pub mod services {
    use super::*;

    pub static DEVICEINFO: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE060010-43E5-11E4-916C-0800200C9A66").unwrap());

    pub static CONTROL: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE060020-43E5-11E4-916C-0800200C9A66").unwrap());
    pub static ROWING: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE060030-43E5-11E4-916C-0800200C9A66").unwrap());
}

pub mod chars {
    use super::*;

    pub static ROWING_GENERAL: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("ce060031-43e5-11e4-916c-0800200c9a66").unwrap());

    pub static FORCECURVE: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE06003D-43E5-11E4-916C-0800200C9A66").unwrap());
}
