#!/bin/bash

# Symlink all input.txt files from a source directory to corresponding directories here
# Usage: ./symlink-inputs.sh [SOURCE_DIR]
# Default: ../advent-inputs

SOURCE_DIR="${1:-../advent-inputs}"

if [ ! -d "$SOURCE_DIR" ]; then
  echo "Error: Directory '$SOURCE_DIR' does not exist"
  exit 1
fi

echo "Symlinking input.txt files from: $SOURCE_DIR"

for file in $(find "$SOURCE_DIR" -name "input.txt" -type f); do
  # Extract year and day from path like ../advent-inputs/2018/day02/input.txt
  # We need to count from the end since the source dir path length varies
  year=$(basename "$(dirname "$(dirname "$file")")")
  day=$(basename "$(dirname "$file")")

  # Create target directory if it doesn't exist
  target_dir="./$year/$day"
  if [ -d "$target_dir" ]; then
    # Create symlink (use absolute path for reliability)
    abs_source=$(cd "$(dirname "$file")" && pwd)/input.txt
    ln -sf "$abs_source" "$target_dir/input.txt"
    echo "Linked: $year/$day/input.txt"
  fi
done
