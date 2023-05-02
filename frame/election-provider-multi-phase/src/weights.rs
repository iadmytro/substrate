// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-w5zkt3rr-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/election-provider-multi-phase/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_election_provider_multi_phase.
pub trait WeightInfo {
	fn on_initialize_nothing() -> Weight;
	fn on_initialize_open_signed() -> Weight;
	fn on_initialize_open_unsigned() -> Weight;
	fn finalize_signed_phase_accept_solution() -> Weight;
	fn finalize_signed_phase_reject_solution() -> Weight;
	fn create_snapshot_internal(v: u32, t: u32, ) -> Weight;
	fn elect_queued(a: u32, d: u32, ) -> Weight;
	fn submit() -> Weight;
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32, ) -> Weight;
}

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CurrentPlannedSession (r:1 w:0)
	/// Proof: Staking CurrentPlannedSession (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Babe EpochIndex (r:1 w:0)
	/// Proof: Babe EpochIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Babe GenesisSlot (r:1 w:0)
	/// Proof: Babe GenesisSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Babe CurrentSlot (r:1 w:0)
	/// Proof: Babe CurrentSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Staking ForceEra (r:1 w:0)
	/// Proof: Staking ForceEra (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `994`
		//  Estimated: `3481`
		// Minimum execution time: 22_945_000 picoseconds.
		Weight::from_parts(23_471_000, 3481)
			.saturating_add(T::DbWeight::get().reads(8_u64))
	}
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1599`
		// Minimum execution time: 12_776_000 picoseconds.
		Weight::from_parts(13_130_000, 1599)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1599`
		// Minimum execution time: 13_788_000 picoseconds.
		Weight::from_parts(14_193_000, 1599)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	fn finalize_signed_phase_accept_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `3593`
		// Minimum execution time: 34_843_000 picoseconds.
		Weight::from_parts(35_573_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn finalize_signed_phase_reject_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `3593`
		// Minimum execution time: 23_486_000 picoseconds.
		Weight::from_parts(24_332_000, 3593)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, _t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 375_567_000 picoseconds.
		Weight::from_parts(397_351_000, 0)
			// Standard Error: 2_663
			.saturating_add(Weight::from_parts(182_990, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `337 + a * (768 ±0) + d * (48 ±0)`
		//  Estimated: `3889 + a * (768 ±0) + d * (49 ±0)`
		// Minimum execution time: 369_614_000 picoseconds.
		Weight::from_parts(38_488_759, 3889)
			// Standard Error: 7_347
			.saturating_add(Weight::from_parts(586_754, 0).saturating_mul(a.into()))
			// Standard Error: 11_013
			.saturating_add(Weight::from_parts(211_374, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
			.saturating_add(Weight::from_parts(0, 768).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(d.into()))
	}
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	/// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `893`
		//  Estimated: `2378`
		// Minimum execution time: 56_019_000 picoseconds.
		Weight::from_parts(59_284_000, 2378)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219 + t * (32 ±0) + v * (553 ±0)`
		//  Estimated: `1704 + t * (32 ±0) + v * (553 ±0)`
		// Minimum execution time: 6_062_043_000 picoseconds.
		Weight::from_parts(6_240_187_000, 1704)
			// Standard Error: 19_561
			.saturating_add(Weight::from_parts(144_938, 0).saturating_mul(v.into()))
			// Standard Error: 57_969
			.saturating_add(Weight::from_parts(5_653_325, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(t.into()))
			.saturating_add(Weight::from_parts(0, 553).saturating_mul(v.into()))
	}
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194 + t * (32 ±0) + v * (553 ±0)`
		//  Estimated: `1679 + t * (32 ±0) + v * (553 ±0)`
		// Minimum execution time: 5_139_280_000 picoseconds.
		Weight::from_parts(5_345_102_000, 1679)
			// Standard Error: 15_671
			.saturating_add(Weight::from_parts(308_566, 0).saturating_mul(v.into()))
			// Standard Error: 46_440
			.saturating_add(Weight::from_parts(3_609_188, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(t.into()))
			.saturating_add(Weight::from_parts(0, 553).saturating_mul(v.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CurrentPlannedSession (r:1 w:0)
	/// Proof: Staking CurrentPlannedSession (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	/// Proof: Staking ErasStartSessionIndex (max_values: None, max_size: Some(16), added: 2491, mode: MaxEncodedLen)
	/// Storage: Babe EpochIndex (r:1 w:0)
	/// Proof: Babe EpochIndex (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Babe GenesisSlot (r:1 w:0)
	/// Proof: Babe GenesisSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Babe CurrentSlot (r:1 w:0)
	/// Proof: Babe CurrentSlot (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Staking ForceEra (r:1 w:0)
	/// Proof: Staking ForceEra (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_nothing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `994`
		//  Estimated: `3481`
		// Minimum execution time: 22_945_000 picoseconds.
		Weight::from_parts(23_471_000, 3481)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
	}
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_signed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1599`
		// Minimum execution time: 12_776_000 picoseconds.
		Weight::from_parts(13_130_000, 1599)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	fn on_initialize_open_unsigned() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1599`
		// Minimum execution time: 13_788_000 picoseconds.
		Weight::from_parts(14_193_000, 1599)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	fn finalize_signed_phase_accept_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `3593`
		// Minimum execution time: 34_843_000 picoseconds.
		Weight::from_parts(35_573_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn finalize_signed_phase_reject_solution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `3593`
		// Minimum execution time: 23_486_000 picoseconds.
		Weight::from_parts(24_332_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	fn create_snapshot_internal(v: u32, _t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 375_567_000 picoseconds.
		Weight::from_parts(397_351_000, 0)
			// Standard Error: 2_663
			.saturating_add(Weight::from_parts(182_990, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn elect_queued(a: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `337 + a * (768 ±0) + d * (48 ±0)`
		//  Estimated: `3889 + a * (768 ±0) + d * (49 ±0)`
		// Minimum execution time: 369_614_000 picoseconds.
		Weight::from_parts(38_488_759, 3889)
			// Standard Error: 7_347
			.saturating_add(Weight::from_parts(586_754, 0).saturating_mul(a.into()))
			// Standard Error: 11_013
			.saturating_add(Weight::from_parts(211_374, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
			.saturating_add(Weight::from_parts(0, 768).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(d.into()))
	}
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
	/// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionIndices (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionIndices (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionNextIndex (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionNextIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SignedSubmissionsMap (r:0 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase SignedSubmissionsMap (max_values: None, max_size: None, mode: Measured)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `893`
		//  Estimated: `2378`
		// Minimum execution time: 56_019_000 picoseconds.
		Weight::from_parts(59_284_000, 2378)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase QueuedSolution (r:1 w:1)
	/// Proof Skipped: ElectionProviderMultiPhase QueuedSolution (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase SnapshotMetadata (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase SnapshotMetadata (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn submit_unsigned(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219 + t * (32 ±0) + v * (553 ±0)`
		//  Estimated: `1704 + t * (32 ±0) + v * (553 ±0)`
		// Minimum execution time: 6_062_043_000 picoseconds.
		Weight::from_parts(6_240_187_000, 1704)
			// Standard Error: 19_561
			.saturating_add(Weight::from_parts(144_938, 0).saturating_mul(v.into()))
			// Standard Error: 57_969
			.saturating_add(Weight::from_parts(5_653_325, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(t.into()))
			.saturating_add(Weight::from_parts(0, 553).saturating_mul(v.into()))
	}
	/// Storage: ElectionProviderMultiPhase DesiredTargets (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase DesiredTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Snapshot (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Snapshot (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase Round (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase Round (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ElectionProviderMultiPhase MinimumUntrustedScore (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase MinimumUntrustedScore (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `v` is `[1000, 2000]`.
	/// The range of component `t` is `[500, 1000]`.
	/// The range of component `a` is `[500, 800]`.
	/// The range of component `d` is `[200, 400]`.
	fn feasibility_check(v: u32, t: u32, a: u32, _d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194 + t * (32 ±0) + v * (553 ±0)`
		//  Estimated: `1679 + t * (32 ±0) + v * (553 ±0)`
		// Minimum execution time: 5_139_280_000 picoseconds.
		Weight::from_parts(5_345_102_000, 1679)
			// Standard Error: 15_671
			.saturating_add(Weight::from_parts(308_566, 0).saturating_mul(v.into()))
			// Standard Error: 46_440
			.saturating_add(Weight::from_parts(3_609_188, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(t.into()))
			.saturating_add(Weight::from_parts(0, 553).saturating_mul(v.into()))
	}
}
