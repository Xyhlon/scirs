//! Example demonstrating feature detection and description functionality
//!
//! This example shows how to:
//! 1. Detect edges using Sobel operator
//! 2. Detect corners using Harris corner detector
//! 3. Extract feature points and compute descriptors

use image::DynamicImage;
use scirs2_vision::feature::{
    detect_and_compute, extract_feature_coordinates, harris_corners, sobel_edges,
};
use scirs2_vision::preprocessing::{gaussian_blur, normalize_brightness};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SciRS2 Vision - Feature Detection Example");

    // In a real application, you would provide your own image file path
    let image_path = "input.jpg"; // Change this to your image path
    println!("Attempting to load image from: {}", image_path);

    // Check if the image file exists
    let path = PathBuf::from(image_path);
    if !path.exists() {
        println!("Image file not found. This example needs an input image.");
        println!("Please provide an image path as argument or place an 'input.jpg' in the current directory.");

        // For demo purposes, we'll create a simple 100x100 gradient image
        println!("Creating a demo gradient image for demonstration...");
        let mut img_buffer = image::ImageBuffer::new(100, 100);

        for y in 0..100 {
            for x in 0..100 {
                let intensity = ((x as f32 / 100.0) * 255.0) as u8;
                img_buffer.put_pixel(x, y, image::Luma([intensity]));
            }
        }

        let img = DynamicImage::ImageLuma8(img_buffer);
        process_image(&img)?;
        return Ok(());
    }

    // Load image
    let img = image::open(path)?;
    println!(
        "Successfully loaded image: {}x{}",
        img.width(),
        img.height()
    );

    process_image(&img)?;

    Ok(())
}

fn process_image(img: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Preprocess the image
    println!("Preprocessing image...");
    let normalized = normalize_brightness(img, 0.0, 1.0)?;
    let blurred = gaussian_blur(&normalized, 1.0)?;

    // 2. Detect edges
    println!("Detecting edges...");
    let edges = sobel_edges(&blurred, 0.1)?;
    println!("Edge detection complete");

    // 3. Detect corners
    println!("Detecting corners...");
    let corners = harris_corners(&blurred, 3, 0.04, 0.01)?;
    println!("Corner detection complete");

    // 4. Extract feature coordinates
    let edge_points = extract_feature_coordinates(&edges);
    let corner_points = extract_feature_coordinates(&corners);

    println!("Detected {} edge points", edge_points.len());
    println!("Detected {} corner points", corner_points.len());

    // 5. Extract features and compute descriptors
    println!("Computing feature descriptors...");
    let descriptors = detect_and_compute(&blurred, 100, 0.1)?;
    println!("Computed {} feature descriptors", descriptors.len());

    // Print some descriptor information
    if !descriptors.is_empty() {
        let desc = &descriptors[0];
        println!(
            "Example keypoint: position=({:.1}, {:.1}), scale={:.1}, orientation={:.1}°, response={:.3}",
            desc.keypoint.x,
            desc.keypoint.y,
            desc.keypoint.scale,
            desc.keypoint.orientation * 180.0 / std::f32::consts::PI,
            desc.keypoint.response
        );

        println!("Descriptor vector length: {}", desc.vector.len());
    }

    println!("Feature processing complete!");

    Ok(())
}
