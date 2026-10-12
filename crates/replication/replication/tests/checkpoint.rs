use lightyear_core::tick::Tick;
use lightyear_replication::checkpoint::{
    CheckpointError, ReplicationCheckpointMap, decode_authoritative_tick, encode_authoritative_tick,
};

use bevy_replicon::prelude::RepliconTick;

extern crate alloc;

#[test]
fn checkpoint_map_prunes_old_entries() {
    let mut map = ReplicationCheckpointMap::default();

    for i in 0..257u32 {
        map.record(RepliconTick::new(i), Tick::from(i));
    }

    assert_eq!(map.get(RepliconTick::new(0)), None);
    assert_eq!(map.get(RepliconTick::new(256)), Some(Tick::from(256)));
}

#[test]
fn the_last_confirmed_checkpoint_outlives_its_pruned_mapping() {
    let mut map = ReplicationCheckpointMap::default();
    map.record(RepliconTick::new(0), Tick::from(10u32));
    let confirmed = map
        .record_last_confirmed_checkpoint(RepliconTick::new(0))
        .unwrap();

    // Newer checkpoints arrive, none of them complete, and the oldest mapping is pruned.
    for i in 1..=256u32 {
        map.record(RepliconTick::new(i), Tick::from(10 + i));
    }
    assert_eq!(map.get(RepliconTick::new(0)), None);

    assert_eq!(
        map.record_last_confirmed_checkpoint(RepliconTick::new(0)),
        Some(confirmed)
    );
    assert_eq!(map.last_confirmed_tick(), Some(Tick::from(10u32)));
}

#[test]
fn authoritative_tick_userdata_roundtrips() {
    let bytes = encode_authoritative_tick(Tick(55));

    assert_eq!(decode_authoritative_tick(&bytes), Ok(Tick(55)));
}

#[test]
fn invalid_authoritative_tick_userdata_is_rejected() {
    assert_eq!(
        decode_authoritative_tick(&[1, 2, 3]),
        Err(CheckpointError::InvalidUserdataLength(3))
    );
}
