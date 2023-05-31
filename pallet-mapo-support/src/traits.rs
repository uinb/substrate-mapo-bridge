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

use crate::{ChainId, XToken};
use codec::{Codec, EncodeLike, MaxEncodedLen};
use frame_support::{traits::BalanceStatus, Parameter};
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member},
    DispatchError, DispatchResult,
};

pub trait Token<AccountId> {
    type Balance: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + Codec
        + MaybeSerializeDeserialize
        + MaxEncodedLen;

    type TokenId: Member
        + Parameter
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + Codec
        + MaybeSerializeDeserialize;

    fn create(token: XToken<Self::Balance>) -> Result<Self::TokenId, DispatchError>;

    fn exists(token_id: &Self::TokenId) -> bool;

    fn native_token_id() -> Self::TokenId;

    fn is_stable(token_id: &Self::TokenId) -> bool;

    fn free_balance(token: &Self::TokenId, who: &AccountId) -> Self::Balance;

    fn total_issuance(token: &Self::TokenId) -> Self::Balance;

    fn transfer_token(
        who: &AccountId,
        token: Self::TokenId,
        amount: Self::Balance,
        receiver: &AccountId,
    ) -> Result<Self::Balance, DispatchError>;

    fn try_mutate_account<R>(
        token: &Self::TokenId,
        who: &AccountId,
        f: impl FnOnce(&mut (Self::Balance, Self::Balance)) -> Result<R, DispatchError>,
    ) -> Result<R, DispatchError>;

    fn try_mutate_issuance(
        token: &Self::TokenId,
        f: impl FnOnce(&mut Self::Balance) -> Result<(), DispatchError>,
    ) -> Result<(), DispatchError>;

    fn token_external_decimals(token: &Self::TokenId) -> Result<u8, DispatchError>;
}

pub trait DecimalsTransformer<Balance> {
    fn transform_decimals_to_standard(amount: Balance, external_decimals: u8) -> Balance;
    fn transform_decimals_to_external(amount: Balance, external_decimals: u8) -> Balance;
}
