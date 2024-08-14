// SPDX-License-Identifier: Commercial
// Copyright 2019 Deepwave Digital, Inc.

/*!
 * Reads GPU frequency and utilization for the integrated GPU of
 * a Jetson TX2 module.
 */

use chrono::prelude::*;
use std::io;

use crate::util;

// The clock rate is given in Hz and requires superuser access to read this file
// (anything in /sys/kernel/debug requires elevated permissions)
const GPU_CLK_FILE: &str = "/sys/kernel/debug/bpmp/debug/clk/gpcclk/rate";

// The load value in sysfs is an integer from 0-1000 as a load percent, value 1000 = 100% load
const GPU_LOAD_FILE_TX2: &str = "/sys/devices/17000000.gp10b/load";
const GPU_LOAD_FILE_XAVIER: &str = "/sys/devices/17000000.gv11b/load";

#[derive(Debug, Clone)]
pub struct Gpu {
    pub timestamp: DateTime<Utc>,
    pub gpu_freq: f32,
    pub gpu_load: f32
}

impl Gpu {
    pub fn new() -> io::Result<Self> {
        if let Ok(gpu_load) = util::value_from_file::<f32, _>(GPU_LOAD_FILE_XAVIER) {
            Ok(Gpu {
                timestamp: Utc::now(),
                gpu_freq:  util::value_from_file(GPU_CLK_FILE)?,
                gpu_load:  gpu_load / 10.0
            })
        } else if let Ok(gpu_load) = util::value_from_file::<f32, _>(GPU_LOAD_FILE_TX2) {
            Ok(Gpu {
                timestamp: Utc::now(),
                gpu_freq:  util::value_from_file(GPU_CLK_FILE)?,
                gpu_load:  gpu_load / 10.0
            })
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Could not read GPU load"))
        }
    }
}
