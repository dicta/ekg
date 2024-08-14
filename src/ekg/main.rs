use std::error::Error;

use ekg_jetson::*;

fn main() -> Result<(), Box<dyn Error>> {
    let therm = Thermal::new();

    let therm_meas_strs : Vec<String> =
        therm.sensors.iter().map(|e| {
            format!("{}={:.2}", &e.sensor_name, &e.temp_degrees)
        }).collect();

    println!("jetson,sensor=jetson_thermal {}", therm_meas_strs.join(","));

    let gpu = Gpu::new()?;

    println!("jetson,sensor=gpu freq={:.2},load={:.3}", gpu.gpu_freq, gpu.gpu_load);

    let emc = Emc::new()?;
    println!("jetson,sensor=mem freq={:.2},overtemp_flag={:.3}", emc.mem_freq, emc.mem_overtemp);

    Ok(())
}
