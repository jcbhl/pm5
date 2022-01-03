use std::lazy::SyncLazy;
use uuid::Uuid;

// PM5 service UUIDS from https://www.c2forum.com/viewtopic.php?f=15&t=81699&p=295721&hilit=uuid#p284373
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

    pub static GENERAL_STATUS: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE060031-43E5-11E4-916C-0800200C9A66").unwrap());

    pub static STATUS_SAMPLE_RATE: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE060034-43E5-11E4-916C-0800200C9A66").unwrap());

    pub static FORCECURVE: SyncLazy<Uuid> =
        SyncLazy::new(|| Uuid::parse_str("CE06003D-43E5-11E4-916C-0800200C9A66").unwrap());
}
