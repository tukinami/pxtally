use image::RgbImage;

use crate::{
    config::OutputArgs,
    counter::{Counter, Filter},
    process::ProcessError,
};

pub(crate) fn output<C, F, T>(
    vec: &[C],
    rgb_image: &RgbImage,
    _filter: &F,
    output_args: &OutputArgs,
    (filtered_total_value, filtered_total_pixel): (f64, u128),
) -> Result<(), ProcessError>
where
    C: Counter,
    F: Filter<T>,
{
    if !output_args.no_io {
        print_count(
            vec,
            rgb_image.width(),
            rgb_image.height(),
            filtered_total_value,
            filtered_total_pixel,
        );
    }

    Ok(())
}

fn print_count<T>(
    vec: &[T],
    width: u32,
    height: u32,
    filtered_total_value: f64,
    filtered_total_pixel: u128,
) where
    T: Counter,
{
    let total_pixel = ((width * height) as f32).max(1.0);

    for counter in vec.iter() {
        let ratio = counter.count() as f32 / total_pixel * 100.0;

        println!(
            "{0:>6.2} -> {1:>6.2} : {2:>6.2}% ({3:>10} px)",
            counter.start(),
            counter.end(),
            ratio,
            counter.count()
        )
    }
    let filtered_avr = filtered_total_value / filtered_total_pixel as f64;
    println!();
    println!(" avr : {0:>8.4}", filtered_avr);
}
