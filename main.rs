extern crate tesseract;

use std::fs;
use tesseract::Tesseract;

fn main() -> Result<(), String> {

    let mut tess = Tesseract::new("eng")?;

    // Set the image file to be processed
    let image_data = fs::read("image.png")?;
    tess.set_image(&image_data)?;

    // Perform OCR and print the result
    let text = tess.text()?;
    println!("{}", text);

    Ok(())
}
