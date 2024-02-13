// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020-2023 Andre Richter <andre.o.richter@gmail.com>

//! Processor code.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

pub mod smp;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::{nop, wait_forever};

#[cfg(feature = "test_build")]
pub use arch_cpu::{qemu_exit_failure, qemu_exit_success};
