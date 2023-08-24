// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

//! Boot code.

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/boot.rs"]
mod arch_boot;
