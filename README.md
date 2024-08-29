# lisudoku_ocr

This is a library for detecting sudoku grids from images.

## Features

* Detects classic sudoku digits and pencilmarks.
* It works on screenshots and probably won't on random photos.

## Installation

It uses OpenCV to detect lines and to split up the puzzle into squares and then Tesseract to detect digits.

You must have `opencv` installed. Follow the steps at https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md.

Download https://github.com/Shreeshrii/tessdata_shreetest/raw/master/digits.traineddata to `/usr/local/share/tessdata/`.

## Running on a local image

`cargo run IMAGE_PATH=src/test_images/image9.png`

## Contribute

Join the [discord server](https://discord.gg/SGV8TQVSeT).

## Running tests

`RUST_TEST_THREADS=1 cargo test`
