// SPDX-License-Identifier: MIT
//
// Copyright (c) 2019-2023 Andre Richter <andre.o.richter@gmail.com>

//! Types for the `custom_test_frameworks` implementation.

#![no_std]

/// Unit test container.
pub struct UnitTest {
    /// Name of the test.
    pub name: &'static str,

    /// Function pointer to the test.
    pub test_func: fn(),
}
