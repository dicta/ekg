// SPDX-License-Identifier: Commercial
// Copyright 2019 Deepwave Digital, Inc.

/*!
 * Reads GPU frequency and utilization for the integrated GPU of
 * a Jetson TX2 module.
 */

use chrono::prelude::*;
use std::io;

use crate::util;

const GPU_CLK_FILE: &str = "/sys/kernel/debug/bpmp/debug/clk/gpcclk/rate";
const GPU_LOAD_FILE: &str = "/sys/devices/17000000.gp10b/load";


#[derive(Debug, Clone)]
pub struct Gpu {
    pub timestamp: DateTime<Utc>,
    pub gpu_freq: f32,
    pub gpu_load: f32
}

impl Gpu {
    pub fn new() -> io::Result<Self> {
        Ok(Gpu {
            timestamp: Utc::now(),
            gpu_freq:  util::value_from_file(GPU_CLK_FILE)?,
            gpu_load:  util::value_from_file(GPU_LOAD_FILE)?
        })
    }
}
