// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Evan Chen <evanchen940628@gmail.com>. All rights reserved.

#[cfg(not(windows))]
compile_error!("batlens-core is Windows-only (requires Windows ACPI/IOCTL and powercfg).");

pub mod stats;
pub mod sys;
pub mod types;
pub mod xml;

#[cfg(test)]
mod tests;
