#!/bin/bash

# First ensure webp package is installed
if ! command -v cwebp &> /dev/null; then
    echo "cwebp not found. Please install webp package:"
    echo "sudo apt install webp    # for Ubuntu/Debian"
    echo "brew install webp        # for macOS"
    exit 1
fi

# Find all PNG files in metadata/android directory and subdirectories
find metadata/android -type f -name "*.png" | while read -r png_file; do
    # Create WebP filename
    webp_file="${png_file%.png}.webp"
    
    # Convert PNG to WebP
    echo "Converting: $png_file"
    cwebp -q 90 "$png_file" -o "$webp_file"
    
    # If conversion successful, remove original PNG
    if [ $? -eq 0 ]; then
        rm "$png_file"
        echo "Converted and removed: $png_file"
    else
        echo "Failed to convert: $png_file"
    fi
done

echo "Conversion completed!"
