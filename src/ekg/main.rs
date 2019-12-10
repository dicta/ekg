use std::error::Error;

use ekg_jetson::*;

fn main() -> Result<(), Box<dyn Error>> {
    let therm = Thermal::new();

    for e in therm.sensors {
        println!("temp,sensor={} temp={:.2}", &e.sensor_name, &e.temp_degrees)
    }

    let gpu = Gpu::new()?;

    println!("gpu,gpu=jetson freq={:.2} load={:.3}", gpu.gpu_freq, gpu.gpu_load);

    Ok(())
}
