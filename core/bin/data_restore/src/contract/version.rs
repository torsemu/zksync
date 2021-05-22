use std::convert::TryFrom;

use crate::{contract, rollup_ops::RollupOpsBlock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkSyncContractVersion {
    V0,
    V1,
    V2,
    V3,
    V4,
}

impl TryFrom<u32> for ZkSyncContractVersion {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use ZkSyncContractVersion::*;

        match value {
            0 => Ok(V0),
            1 => Ok(V1),
            2 => Ok(V2),
            3 => Ok(V3),
            4 => Ok(V4),
            _ => Err(anyhow::anyhow!("Unsupported contract version")),
        }
    }
}
impl From<ZkSyncContractVersion> for i32 {
    fn from(val: ZkSyncContractVersion) -> Self {
        match val {
            ZkSyncContractVersion::V0 => 0,
            ZkSyncContractVersion::V1 => 1,
            ZkSyncContractVersion::V2 => 2,
            ZkSyncContractVersion::V3 => 3,
            ZkSyncContractVersion::V4 => 4,
        }
    }
}

impl ZkSyncContractVersion {
    pub fn rollup_ops_blocks_from_bytes(
        &self,
        data: Vec<u8>,
    ) -> anyhow::Result<Vec<RollupOpsBlock>> {
        use ZkSyncContractVersion::*;
        let mut blocks = match self {
            V0 | V1 | V2 | V3 => vec![contract::default::rollup_ops_blocks_from_bytes(data)?],
            V4 => contract::v4::rollup_ops_blocks_from_bytes(data)?,
        };
        // Set the contract version.
        for block in blocks.iter_mut() {
            block.contract_version = Some(*self);
        }
        Ok(blocks)
    }

    /// Increment the contract version by one.
    ///
    /// # Panics
    ///
    /// Panics if the version is already latest.
    pub fn upgrade(&mut self) {
        *self = Self::try_from(i32::from(*self) as u32 + 1)
            .expect("cannot upgrade latest contract version");
    }

    /// Returns supported block chunks sizes by the verifier contract
    /// with the given version.
    pub fn available_block_chunk_sizes(&self) -> &'static [usize] {
        match self {
            _ => &[10, 32, 72, 156, 322, 654],
        }
    }
}
