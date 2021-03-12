use std::{convert::TryInto as _, fs, mem};

use anyhow::anyhow;
use image::{DynamicImage, GenericImageView as _, ImageBuffer};

fn main() -> Result<(), anyhow::Error> {
    fs::read("ignucius.jpg")
        .map_err(|e| anyhow!("Failed to read file: {}", e))
        .and_then(|file| decode_jpeg(&file))
        .and_then(|image| encode_jpeg(image))
        .map(|_| ())
}

fn encode_jpeg(image: DynamicImage) -> Result<Vec<u8>, anyhow::Error> {
    let (width, height) = image.dimensions();

    let mut encoder = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    encoder.set_mem_dest();
    encoder.set_size(width.try_into().unwrap(), height.try_into().unwrap());

    encoder.set_color_space(mozjpeg::ColorSpace::JCS_YCbCr);
    {
        let mut comp = encoder.components_mut();
        let (h, v) = (2, 2); // CbCr420 subsampling factors
                             // 0 - Y, 1 - Cb, 2 - Cr, 3 - K
        comp[1].h_samp_factor = h;
        comp[1].v_samp_factor = v;
        comp[2].h_samp_factor = h;
        comp[2].v_samp_factor = v;
    }

    encoder.start_compress();
    let _ = encoder.write_scanlines(image.as_bytes());
    encoder.finish_compress();

    encoder
        .data_to_vec()
        .map_err(|_| anyhow!("Failed to encode JPEG as bytes"))
}

fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage, anyhow::Error> {
    let mut decoder = match mozjpeg::Decompress::new_mem(buffer)?.image()? {
        mozjpeg::decompress::Format::RGB(d) => d,
        _ => unimplemented!(),
    };

    let width = decoder.width().try_into().unwrap();
    let height = decoder.height().try_into().unwrap();

    let image = decoder
        .read_scanlines::<[u8; 3]>()
        .ok_or(anyhow!("decoder.read_scanlines() failed"))?;

    Ok(DynamicImage::ImageRgb8(
        ImageBuffer::from_raw(width, height, into_vec_u8(image)).unwrap(),
    ))
}

/// Casts `Vec<T>` into `Vec<u8>`.
fn into_vec_u8<T: Copy>(mut buffer: Vec<T>) -> Vec<u8> {
    // TODO: Change to `Vec::into_raw_parts` once it gets stabilized.
    let ptr = buffer.as_mut_ptr();
    let len = buffer.len();
    let cap = buffer.capacity();
    mem::forget(buffer);

    let sz = mem::size_of::<T>();

    // This is safe, because `T` is `Copy`, so `!Drop`, and we ensured that
    // allocated memory is still in place using [`mem::forget`].
    // This is also safe as long as `T` is equally aligned with `u8`.
    unsafe { Vec::from_raw_parts(ptr as *mut u8, len * sz, cap * sz) }
}
