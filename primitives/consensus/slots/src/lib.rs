// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Primitives for slots-based consensus engines.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

/// Unit type wrapper that represents a slot.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Slot(pub u64);

impl Deref for Slot {
	type Target = u64;

	fn deref(&self) -> &u64 {
		&self.0
	}
}

impl fmt::Display for Slot {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// Represents an equivocation proof. An equivocation happens when a validator
/// produces more than one block on the same slot. The proof of equivocation
/// are the given distinct headers that were signed by the validator and which
/// include the slot number.
#[derive(Clone, Debug, Decode, Encode, PartialEq)]
pub struct EquivocationProof<Header, Id> {
	/// Returns the authority id of the equivocator.
	pub offender: Id,
	/// The slot at which the equivocation happened.
	pub slot: Slot,
	/// The first header involved in the equivocation.
	pub first_header: Header,
	/// The second header involved in the equivocation.
	pub second_header: Header,
}
