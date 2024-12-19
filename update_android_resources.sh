#!/bin/bash
# Source directories
METADATA_DIR="metadata/android"
MANIFEST_SOURCE="$METADATA_DIR/AndroidManifest.xml"
RES_SOURCE="$METADATA_DIR/res"
GRADLE_SOURCE="$METADATA_DIR/build.gradle.kts"
ACTIVITY_SOURCE="$METADATA_DIR/MainActivity.kt"

# Target directories
RELEASE_DIR="target/dx/myquest/release/android/app/app/src/main"
RELEASE_GRADLE="target/dx/myquest/release/android/app/app"
KOTLIN_TARGET="$RELEASE_DIR/kotlin/dev/dioxus/main"

# Function to copy files
copy_android_resources() {
    local target_dir="$1"
    local gradle_dir="$2"
    local kotlin_target="$3"

    # Create directories if they don't exist
    mkdir -p "$target_dir"
    mkdir -p "$gradle_dir"
    mkdir -p "$kotlin_target"

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
        rm -rf "$target_dir/res"
        cp -r "$RES_SOURCE" "$target_dir/"
        echo "Copied res folder to $target_dir"
    else
        echo "Error: Source res directory not found at $RES_SOURCE"
        exit 1
    fi

    # Copy build.gradle.kts
    if [ -f "$GRADLE_SOURCE" ]; then
        cp "$GRADLE_SOURCE" "$gradle_dir/build.gradle.kts"
        echo "Copied build.gradle.kts to $gradle_dir"
    else
        echo "Error: Source build.gradle.kts not found at $GRADLE_SOURCE"
        exit 1
    fi

    # Copy MainActivity.kt
    if [ -f "$ACTIVITY_SOURCE" ]; then
        cp "$ACTIVITY_SOURCE" "$kotlin_target/MainActivity.kt"
        echo "Copied MainActivity.kt to $kotlin_target"
    else
        echo "Error: Source MainActivity.kt not found at $ACTIVITY_SOURCE"
        exit 1
    fi
}

# Copy to release directory
copy_android_resources "$RELEASE_DIR" "$RELEASE_GRADLE" "$KOTLIN_TARGET"
echo "Resource copy completed successfully!"