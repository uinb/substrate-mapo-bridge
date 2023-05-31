// Copyright 2021-2023 UINB Technologies Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Codec, Decode, Encode};
use core::fmt::Debug;
use frame_support::dispatch::Dispatchable;
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec::Vec};
pub type ChainId = u32;

pub mod mapolightclient {
    use crate::ChainId;
    use alloc::string::{String, ToString};
    use sp_std::vec::Vec;

    pub type DepositNonce = u64;
    pub type ResourceId = [u8; 32];
    pub type EvmHash = [u8; 32];
    pub type EthereumCompatibleAddress = [u8; 20];

    pub trait AssetIdResourceIdProvider<TokenId> {
        type Err;

        fn try_get_asset_id(
            chain_id: ChainId,
            contract_id: impl AsRef<[u8]>,
        ) -> Result<TokenId, Self::Err>;
    }

    pub struct  TokenInfo {
        pub contract_addr: EthereumCompatibleAddress,
        pub origin_chain_id: ChainId,
        pub deciamls: u8,
        pub symbol: String,
    }
}
