#!/bin/bash

# Source directories
METADATA_DIR="metadata/android"
MANIFEST_SOURCE="$METADATA_DIR/AndroidManifest.xml"
RES_SOURCE="$METADATA_DIR/res"

# Target directories
DEBUG_DIR="target/dx/myquest/debug/android/app/app/src/main"
RELEASE_DIR="target/dx/myquest/release/android/app/app/src/main"

# Function to copy files
copy_android_resources() {
    local target_dir="$1"
    
    # Create directories if they don't exist
    mkdir -p "$target_dir"
    
    # Copy AndroidManifest.xml
    if [ -f "$MANIFEST_SOURCE" ]; then
        cp "$MANIFEST_SOURCE" "$target_dir/AndroidManifest.xml"
        echo "Copied AndroidManifest.xml to $target_dir"
    else
        echo "Error: Source AndroidManifest.xml not found at $MANIFEST_SOURCE"
        exit 1
    fi
    
    # Copy res directory
    if [ -d "$RES_SOURCE" ]; then
        # Remove existing res directory if it exists
        rm -rf "$target_dir/res"
        cp -r "$RES_SOURCE" "$target_dir/"
        echo "Copied res folder to $target_dir"
    else
        echo "Error: Source res directory not found at $RES_SOURCE"
        exit 1
    fi
}

# Copy to debug directory
copy_android_resources "$DEBUG_DIR"

# Copy to release directory
copy_android_resources "$RELEASE_DIR"

echo "Resource copy completed successfully!"
