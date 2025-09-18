use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, ImageFormat};
use std::path::{Path, PathBuf};
use anyhow::{Context, Ok, Result};

#[derive(Parser)]
#[command(name = "img-resize")]
#[command(about = "A simple image resizing tool")]
#[command(version = "0.1.0")]
struct Args {
    /// Input image file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file path (optional, defaults to input with _resized suffix)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Target width in pixels
    #[arg(short, long)]
    width: Option<u32>,

    /// Target height in pixels
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Scale factor (e.g., 0.5 for half size, 2.0 for double size)
    #[arg(short, long)]
    scale: Option<f32>,

    /// Resize filter algorithm
    #[arg(short = 'F', long, default_value = "lanczos3")]
    filter: Filter,

    /// Maintain aspect ratio when only width or height is specified
    #[arg(short, long, default_value = "true")]
    aspect_ratio: bool,

    /// Output format (auto-detected from extension if not specified)
    #[arg(short = 'f', long)]
    format: Option<Format>,

    /// JPEG quality (1-100, only applies to JPEG output)
    #[arg(short, long, default_value = "90")]
    quality: u8,
}

// ValueEnum: Allows this enum to be used with Clap (a command-line argument parser) so you can pass these options via CLI arguments like --filter nearest.
// Clone: Lets you duplicate values of Filter.
// Debug: Enables the {:?} formatter for debugging output.
#[derive(ValueEnum, Clone, Debug)]
enum Filter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

// This implements the From trait, allowing you to easily convert a Filter into a FilterType.
// match is used to map each variant of Filter to the corresponding variant of FilterType.
impl From<Filter> for FilterType {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Nearest => FilterType::Nearest,
            Filter::Triangle => FilterType::Triangle,
            Filter::CatmullRom => FilterType::CatmullRom,
            Filter::Gaussian => FilterType::Gaussian,
            Filter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Png,
    Jpeg,
    Jpg,
    Webp,
    Bmp,
    Tiff,
}

impl From<Format> for ImageFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Png => ImageFormat::Png,
            Format::Jpeg | Format::Jpg => ImageFormat::Jpeg,
            Format::Webp => ImageFormat::WebP,
            Format::Bmp => ImageFormat::Bmp,
            Format::Tiff => ImageFormat::Tiff,
        }
    }
}

// cargo run -- -i image.webp -w 800
fn main() -> Result<()>{
    let args = Args::parse();

    // Cleaner code: You don't have to write return Err(...) explicitly.
    // Automatic conversion: anyhow can wrap many error types, making error handling simpler.
    if args.width.is_none() && args.height.is_none() && args.scale.is_none() {
        anyhow::bail!("Error: Must specify at least one of --width, --height, or --scale.");

        // return Err(anyhow!(
        //     "Error: Must specify at least one of --width, --height, or --scale."
        // ));
    }

    if args.quality < 1 || args.quality > 100 {
        anyhow::bail!("Error: Quality must be between 1 and 100.");
    }

    // Load the image from the input file.
    let img = image::open(&args.input).with_context(
        || format!("Failed to open input image: {:?}", args.input.display()))?;

    println!("Loaded image: {}x{}", img.width(), img.height());

    // Calculate new dimensions.
    let (target_width, target_height) = calculate_dimensions(
        img.width(),
        img.height(),
        args.width,
        args.height,
        args.scale,
        args.aspect_ratio,
    )?;

    println!("Resizing to: {}x{}", target_width, target_height);

    // Resize the image using the specified filter.
    let resized = img.resize(target_width, target_height, args.filter.into());

    // Determine output path.
    let output_path = match args.output {
        Some(path) => path,
        None => generate_output_path(&args.input)?,
    };

    // Determine output format.
    let format = match args.format {
        Some(fmt) => fmt.into(),
        None => detect_format_from_path(&output_path)?,
    };

    // Save the image
    match format {
        ImageFormat::Jpeg => {
            // NOTE: only JPEG needs special handling for quality

            // Use custom JPEG encoder for quality control
            // It gives fine-grained control over JPEG quality, which isn't possible when using the generic save() method.
            let mut buffer = Vec::new();

            // Creates a custom JPEG encoder and sets the quality level using args.quality.
            // Quality typically ranges from 1 (worst) to 100 (best).
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, args.quality);

            // Encodes the resized image using the custom encoder and writes the result into buffer.
            resized.write_with_encoder(encoder)
                .context("Failed to encode JPEG")?;
            std::fs::write(&output_path, buffer)
                .with_context(|| format!("Failed to write to {}", output_path.display()))?;
        }
        _ => {
            resized.save_with_format(&output_path, format)
                .with_context(|| format!("Failed to save image to {}", output_path.display()))?;
        }
    }

    println!("Image saved to: {}", output_path.display());

    Ok(())
}

fn calculate_dimensions(
    orig_width: u32,
    orig_height: u32,
    target_width: Option<u32>,
    target_height: Option<u32>,
    scale: Option<f32>,
    maintain_aspect: bool,
) -> Result<(u32, u32)> {
    match (target_width, target_height, scale) {
        // Scale factor provided
        (None, None, Some(s)) => {
            if s <= 0.0 {
                anyhow::bail!("Scale factor must be positive");
            }
            Ok(((orig_width as f32 * s) as u32, (orig_height as f32 * s) as u32))
        }

        // Both dimensions provided
        (Some(w), Some(h), None) => Ok((w, h)),

        // Only width provided
        (Some(w), None, None) => {
            if maintain_aspect {
                let aspect_ratio = orig_height as f32 / orig_width as f32;
                let h = (w as f32 * aspect_ratio) as u32;
                Ok((w, h))
            } else {
                anyhow::bail!("Height must be specified when aspect ratio is not maintained");
            }
        }

        // Only height provided
        (None, Some(h), None) => {
            if maintain_aspect {
                let aspect_ratio = orig_width as f32 / orig_height as f32;
                let w = (h as f32 * aspect_ratio) as u32;
                Ok((w, h))
            } else {
                anyhow::bail!("Width must be specified when aspect ratio is not maintained");
            }
        }

        // Invalid combinations
        _ => anyhow::bail!("Invalid combination of width, height, and scale parameters"),
    }
}

fn generate_output_path(input: &Path) -> Result<PathBuf> {
    // file_stem() extracts the filename without its extension.
    let stem = input.file_stem()
        .context("Input file has no filename")? // if file name is none
        .to_str() // file_stem() gives OsStr, which might not always be valid UTF-8.
        .context("Invalid filename")?; // if filename is not valid UTF-8

    let extension = input.extension()
        .and_then(|ext| ext.to_str()) // .and_then(|ext| ext.to_str()) → Converts it to &str
        .unwrap_or("png");

    let parent = input.parent().unwrap_or(Path::new("."));

    Ok(parent.join(format!("{}_resized.{}", stem, extension)))
}

fn detect_format_from_path(path: &Path) -> Result<ImageFormat> {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .context("No file extension found")?
        .to_lowercase();

    match extension.as_str() {
        "png" => Ok(ImageFormat::Png),
        "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
        "webp" => Ok(ImageFormat::WebP),
        "bmp" => Ok(ImageFormat::Bmp),
        "tiff" | "tif" => Ok(ImageFormat::Tiff),
        _ => anyhow::bail!("Unsupported image format: {}", extension),
    }
}