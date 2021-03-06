//! Concread - Concurrently Readable Datastructures
//!
//! Concurrently readable is often referred to as Copy-On-Write, Multi-Version-Concurrency-Control.
//!
//! These structures allow multiple readers with transactions
//! to proceed while single writers can operate. A reader is guaranteed the content
//! will remain the same for the duration of the read, and readers do not block writers.
//! Writers are serialised, just like a mutex.
//!
//! You can use these in place of a RwLock, and will likely see improvements in
//! parallel throughput.
//!
//! The best use is in place of mutex/rwlock, where the reader exists for a
//! non-trivial amount of time.
//!
//! For example, if you have a RwLock where the lock is taken, data changed or read, and dropped
//! immediately, this probably won't help you.
//!
//! However, if you have a RwLock where you hold the read lock for any amount of time,
//! writers will begin to stall - or inversely, the writer will cause readers to block
//! and wait as the writer proceeds.
//!
//! In the future, a concurrent BTree and HashTree will be added, that can be used inplace
//! of a `RwLock<BTreeMap>` or `RwLock<HashMap>`. Stay tuned!

// #![deny(warnings)]
#![warn(unused_extern_crates)]
#![warn(missing_docs)]

extern crate ahash;
extern crate crossbeam;
extern crate crossbeam_epoch;
extern crate libc;
extern crate parking_lot;
extern crate rand;
extern crate smallvec;

pub mod cache;
pub mod collections;
pub mod cowcell;
pub mod ebrcell;
#[cfg(test)]
extern crate crossbeam_utils;
#[cfg(test)]
mod lincowcell;

pub use cowcell::CowCell;
pub use ebrcell::EbrCell;
