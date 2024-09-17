pub mod collection;
pub mod collection_manager;
pub mod collection_state;
pub mod common;
pub mod config;
pub mod discovery;
pub mod grouping;
pub mod hash_ring;
pub mod lookup;
pub mod operations;
pub mod optimizers_builder;
pub mod recommendations;
pub mod save_on_disk;
pub mod shards;
pub mod telemetry;
mod update_handler;
pub mod wal;
pub mod wal_delta;

pub mod events;
#[cfg(test)]
mod tests;

// TODO: move this somewhere else?
pub type BitSliceElement = u64;
pub type BitSlice = bitvec::slice::BitSlice<BitSliceElement, bitvec::order::Lsb0>;
pub type BitVec = bitvec::vec::BitVec<BitSliceElement, bitvec::order::Lsb0>;
