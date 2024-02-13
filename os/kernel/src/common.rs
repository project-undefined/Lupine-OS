// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020-2023 Andre Richter <andre.o.richter@gmail.com>

//! General purpose code.

/// Convert a size into human readable format.
pub const fn size_human_readable_ceil(size: usize) -> (usize, &'static str) {
    const KIB: usize = 1024;
    const MIB: usize = 1024 * 1024;
    const GIB: usize = 1024 * 1024 * 1024;

    if (size / GIB) > 0 {
        (size.div_ceil(GIB), "GiB")
    } else if (size / MIB) > 0 {
        (size.div_ceil(MIB), "MiB")
    } else if (size / KIB) > 0 {
        (size.div_ceil(KIB), "KiB")
    } else {
        (size, "Byte")
    }
}
