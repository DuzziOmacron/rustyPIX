extern crate image;

use image::{ImageBuffer, Rgb};
use rand::random;
use rustacuda::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize CUDA
    rustacuda::init(CudaFlags::empty())?;

    // Get the first available device
    let device = Device::get_device(0)?;

    // Create a CUDA context
    let _context = Context::create_and_push(
        ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO,
        device,
    )?;

    // Specify the dimensions of the image
    let width = 512;
    let height = 512;

    // Create a new blank image with the specified dimensions
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    // Allocate GPU memory for the image
//    let mut image_gpu = DeviceBuffer::new(image.as_raw())?;
      let mut image_gpu = DeviceBuffer::new(image.into_raw())?;

    // Iterate over each pixel in the image
    for y in 0..height {
        for x in 0..width {
            // Get the neighboring pixels' values from the GPU memory
            let left = image_gpu.get_pixel(x.saturating_sub(1), y)?;
            let up = image_gpu.get_pixel(x, y.saturating_sub(1))?;

            // Generate a random color
            let mut color = Rgb([0, 0, 0]);
            while color == left || color == up {
                color = Rgb([
                    random::<u8>(),
                    random::<u8>(),
                    random::<u8>(),
                ]);
            }

            // Set the current pixel to the generated color
            image.put_pixel(x, y, color);

            // Copy the updated pixel value back to the GPU memory
            image_gpu.set_pixel(x, y, color)?;
        }
    }

    // Copy the final image from GPU memory to host memory
    image_gpu.copy_to(&mut image)?;

    // Save the image as a JPEG file
    image.save("output.jpg")?;

    Ok(())
}






/*
extern crate image;

use image::{ImageBuffer, Rgb};

fn main() {
    // Specify the dimensions of the image
    let width = 512;
    let height = 512;

    // Create a new blank image with the specified dimensions
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    // Iterate over each pixel in the image
    for y in 0..height {
        for x in 0..width {
            // Get the neighboring pixels' values
            let left = image.get_pixel(x.saturating_sub(1), y);
            let up = image.get_pixel(x, y.saturating_sub(1));

            // Generate a random color
            let mut color = Rgb([0, 0, 0]);
            while color == left || color == up {
                color = Rgb([
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                ]);
            }

            // Set the current pixel to the generated color
            image.put_pixel(x, y, color);
        }
    }

    // Save the image as a PNG file
    image.save("output.png").expect("Failed to save image");
}

*/


