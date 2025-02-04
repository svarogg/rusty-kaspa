use std::sync::Arc;

use super::{
    caching::{CachedDbAccess, CachedDbAccessForCopy},
    errors::StoreError,
    DB,
};
use consensus_core::header::Header;
use hashes::Hash;
use rocksdb::WriteBatch;
use serde::{Deserialize, Serialize};

pub trait HeaderStoreReader {
    fn get_daa_score(&self, hash: Hash) -> Result<u64, StoreError>;
    fn get_timestamp(&self, hash: Hash) -> Result<u64, StoreError>;
    fn get_header(&self, hash: Hash) -> Result<Arc<Header>, StoreError>;
}

pub trait HeaderStore: HeaderStoreReader {
    // This is append only
    fn insert(&self, hash: Hash, header: Arc<Header>) -> Result<(), StoreError>;
}

const HEADERS_STORE_PREFIX: &[u8] = b"headers";
const COPMACT_HEADER_DATA_STORE_PREFIX: &[u8] = b"compact-header-data";

#[derive(Clone, Copy, Serialize, Deserialize)]
struct CompactHeaderData {
    daa_score: u64,
    timestamp: u64,
}

/// A DB + cache implementation of `HeaderStore` trait, with concurrency support.
#[derive(Clone)]
pub struct DbHeadersStore {
    raw_db: Arc<DB>,
    // `CachedDbAccess` is shallow cloned so no need to wrap with Arc
    cached_compact_headers_access: CachedDbAccessForCopy<Hash, CompactHeaderData>,
    cached_headers_access: CachedDbAccess<Hash, Header>,
}

impl DbHeadersStore {
    pub fn new(db: Arc<DB>, cache_size: u64) -> Self {
        Self {
            raw_db: Arc::clone(&db),
            cached_compact_headers_access: CachedDbAccessForCopy::new(
                Arc::clone(&db),
                cache_size,
                COPMACT_HEADER_DATA_STORE_PREFIX,
            ),
            cached_headers_access: CachedDbAccess::new(Arc::clone(&db), cache_size, HEADERS_STORE_PREFIX),
        }
    }

    pub fn clone_with_new_cache(&self, cache_size: u64) -> Self {
        Self {
            raw_db: Arc::clone(&self.raw_db),
            cached_compact_headers_access: CachedDbAccessForCopy::new(
                Arc::clone(&self.raw_db),
                cache_size,
                COPMACT_HEADER_DATA_STORE_PREFIX,
            ),
            cached_headers_access: CachedDbAccess::new(Arc::clone(&self.raw_db), cache_size, HEADERS_STORE_PREFIX),
        }
    }

    pub fn insert_batch(&self, batch: &mut WriteBatch, hash: Hash, header: Arc<Header>) -> Result<(), StoreError> {
        if self.cached_headers_access.has(hash)? {
            return Err(StoreError::KeyAlreadyExists(hash.to_string()));
        }
        self.cached_headers_access
            .write_batch(batch, hash, &header)?;
        self.cached_compact_headers_access.write_batch(
            batch,
            hash,
            CompactHeaderData { daa_score: header.daa_score, timestamp: header.timestamp },
        )?;
        Ok(())
    }
}

impl HeaderStoreReader for DbHeadersStore {
    fn get_daa_score(&self, hash: Hash) -> Result<u64, StoreError> {
        if let Some(header) = self.cached_headers_access.read_from_cache(hash) {
            return Ok(header.daa_score);
        }
        Ok(self
            .cached_compact_headers_access
            .read(hash)?
            .daa_score)
    }

    fn get_timestamp(&self, hash: Hash) -> Result<u64, StoreError> {
        if let Some(header) = self.cached_headers_access.read_from_cache(hash) {
            return Ok(header.timestamp);
        }
        Ok(self
            .cached_compact_headers_access
            .read(hash)?
            .timestamp)
    }

    fn get_header(&self, hash: Hash) -> Result<Arc<Header>, StoreError> {
        self.cached_headers_access.read(hash)
    }
}

impl HeaderStore for DbHeadersStore {
    fn insert(&self, hash: Hash, header: Arc<Header>) -> Result<(), StoreError> {
        if self.cached_headers_access.has(hash)? {
            return Err(StoreError::KeyAlreadyExists(hash.to_string()));
        }
        self.cached_compact_headers_access
            .write(hash, CompactHeaderData { daa_score: header.daa_score, timestamp: header.timestamp })?;
        self.cached_headers_access.write(hash, &header)?;
        Ok(())
    }
}
