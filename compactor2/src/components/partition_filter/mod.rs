use std::fmt::{Debug, Display};

use async_trait::async_trait;
use data_types::{ParquetFile, PartitionId};

use crate::error::DynError;

pub mod and;
pub mod greater_matching_files;
pub mod has_files;
pub mod has_matching_file;
pub mod logging;
pub mod max_files;
pub mod max_parquet_bytes;
pub mod metrics;
pub mod never_skipped;
pub mod or;

/// Filters partition based on ID and parquet files.
///
/// May return an error. In this case, the partition will be marked as "skipped".
///
/// If you only plan to inspect the ID but not the files and not perform any IO, check
/// [`IdOnlyPartitionFilter`](crate::components::id_only_partition_filter::IdOnlyPartitionFilter) which usually runs
/// earlier in the pipeline and hence is more efficient.
#[async_trait]
pub trait PartitionFilter: Debug + Display + Send + Sync {
    async fn apply(
        &self,
        partition_id: PartitionId,
        files: &[ParquetFile],
    ) -> Result<bool, DynError>;
}

// Simple Partitions filters for testing purposes

/// True partition filter.
#[derive(Debug)]
pub struct TruePartitionFilter;

impl Display for TruePartitionFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "true")
    }
}

#[async_trait]
impl PartitionFilter for TruePartitionFilter {
    async fn apply(
        &self,
        _partition_id: PartitionId,
        _files: &[ParquetFile],
    ) -> Result<bool, DynError> {
        Ok(true)
    }
}

impl TruePartitionFilter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}

/// False partition filter.
#[derive(Debug)]
pub struct FalsePartitionFilter;

impl Display for FalsePartitionFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "false")
    }
}

#[async_trait]
impl PartitionFilter for FalsePartitionFilter {
    async fn apply(
        &self,
        _partition_id: PartitionId,
        _files: &[ParquetFile],
    ) -> Result<bool, DynError> {
        Ok(false)
    }
}

impl FalsePartitionFilter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }
}
