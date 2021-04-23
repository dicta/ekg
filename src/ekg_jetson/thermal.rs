// SPDX-License-Identifier: Commercial
// Copyright 2019 Deepwave Digital, Inc.

/*!
 * Reads temperature values for one or more thermal zones in the system.
 * Thermal zones are defined by the devicetree for ARM systems and have
 * been defined for each Jetson module variant. Sensor measurements are
 * available in sysfs for each named thermal zone with a reported
 * accuracy of +/- 0.5C (per the NVIDIA Tegra documentation).
 */

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::prelude::*;

use crate::util;


#[derive(Debug, Clone)]
pub struct Thermal {
    pub timestamp: DateTime<Utc>,
    pub sensors: Vec<ThermalSensorMeasurement>,
}

#[derive(Debug, Clone)]
pub struct ThermalSensorMeasurement {
    pub sensor_name: String,
    pub temp_degrees: f32
}

impl Thermal {
    pub fn new() -> Self {
        Thermal {
            timestamp: Utc::now(),
            sensors: get_thermal_zones()
                        .into_iter()
                        .map(ThermalSensorMeasurement::new)
                        .filter_map(Result::ok)
                        .collect()
        }
    }
}

impl ThermalSensorMeasurement {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let name = util::string_from_file(path.as_ref().join("type"))?;
        let temp : i32 = util::value_from_file(path.as_ref().join("temp"))?;

        Ok(ThermalSensorMeasurement {
            sensor_name:  name,
            temp_degrees: temp as f32 / 1000.0
        })
    }
}

fn get_thermal_zones() -> Vec<PathBuf> {
    let thermal_root = Path::new("/sys/devices/virtual/thermal");
    fs::read_dir(thermal_root).unwrap()
        .map(|e| e.unwrap())
        .filter(|e| { e.path().is_dir() &&
                      e.file_name().to_string_lossy().starts_with("thermal_zone") })
        .map(|e| e.path())
        .collect()
}
