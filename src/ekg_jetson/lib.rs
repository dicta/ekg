// SPDX-License-Identifier: Commercial
// Copyright 2019 Deepwave Digital, Inc.

/*!
 * This library contains interfaces to access hardware utilization
 * information on the NVIDIA Jetson series of embedded compute modules.
 */

extern crate chrono;

pub mod gpu;
pub mod mem;
pub mod thermal;
pub mod util;

pub use self::gpu::Gpu;
pub use self::mem::Emc;
pub use self::thermal::Thermal;
pub use self::util::*;
