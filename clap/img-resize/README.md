## Key Features:

1. **Multiple resize options**:
   - Resize by width and/or height
   - Resize by scale factor (e.g., 0.5 for half size)
   - Maintain aspect ratio option

2. **Image format support**:
   - PNG, JPEG, WebP, BMP, TIFF
   - Auto-detection from file extension
   - Manual format specification

3. **Quality control**:
   - JPEG quality setting (1-100)
   - Multiple resize filters (Nearest, Triangle, CatmullRom, Gaussian, Lanczos3)

4. **User-friendly CLI**:
   - Clear help messages
   - Automatic output filename generation
   - Progress feedback

## Usage Examples:

```bash
# Build the project
cargo build --release

# Resize by width (maintains aspect ratio)
./target/release/img-resize -i photo.jpg -w 800

# Resize by exact dimensions
./target/release/img-resize -i photo.jpg -w 800 -H 600 --aspect-ratio false

# Scale by factor
./target/release/img-resize -i photo.jpg -s 0.5

# Specify output file and quality
./target/release/img-resize -i photo.jpg -w 1200 -o resized_photo.jpg -q 95

# Use different filter
./target/release/img-resize -i photo.jpg -w 800 --filter gaussian
```

## Command Line Arguments:

- `-i, --input`: Input image file path
- `-o, --output`: Output path (optional, auto-generated if not provided)
- `-w, --width`: Target width in pixels
- `-H, --height`: Target height in pixels
- `-s, --scale`: Scale factor
- `-f, --filter`: Resize algorithm (lanczos3, nearest, triangle, etc.)
- `-a, --aspect-ratio`: Maintain aspect ratio (default: true)
- `--format`: Output format
- `-q, --quality`: JPEG quality (1-100)

The tool includes proper error handling, validation, and informative output messages. It automatically detects image formats and generates sensible output filenames when not specified.