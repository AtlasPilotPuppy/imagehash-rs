// Copyright 2024 Shun Takebayashi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The `imagehash` crate provides image hashing algorithms.
//!
//! ## Supported Algorithms
//!
//! - Average Hash (aHash)
//! - Difference Hash (dHash)
//!
//! ## Usage
//!
//! ```rust
//! use image;
//! use imagehash::AverageHash;
//!
//! let img_filename = "tests/1.jpg";
//! let img = image::open(img_filename).unwrap();
//!
//! let hasher = AverageHash::default();
//! let hash = hasher.hash(&img);
//! println!("{}", hash); // hex-encoded hash string
//! ```

pub use image::imageops::FilterType;

/// Contains the image pre-processing parameters.
pub struct ImageOp {
    pub width: u8,
    pub height: u8,
    pub filter: FilterType,
}

/// Provides average hash (aHash) calculation.
pub struct AverageHash<'a> {
    op: &'a ImageOp,
}

impl<'a> AverageHash<'a> {
    /// Creates a new `AverageHasher` with default parameters.
    pub fn new() -> Self {
        AverageHash::default()
    }

    /// Creates a new `AverageHasher` with the specified parameters.
    pub fn with_op(op: &'a ImageOp) -> Self {
        AverageHash { op }
    }

    /// Calculates average hash (aHash) of the image and returns as a hex string.
    pub fn hash(&self, image: &image::DynamicImage) -> String {
        let bits = average_hash(image, self.op);
        let bytes = bits_to_bytes(&bits);
        bytes_to_hex(&bytes)
    }
}

impl Default for AverageHash<'_> {
    /// Creates a new `AverageHasher` with default parameters.
    fn default() -> Self {
        AverageHash {
            op: &ImageOp {
                width: 8,
                height: 8,
                filter: FilterType::Lanczos3,
            },
        }
    }
}

/// Calculates average hash (aHash) of the image.
pub fn average_hash(image: &image::DynamicImage, op: &ImageOp) -> Vec<bool> {
    let preprocessed = image
        .grayscale()
        .resize_exact(op.width as u32, op.height as u32, op.filter);
    let pixels = preprocessed.into_luma8().into_raw();
    let average = pixels.iter().map(|i| u16::from(*i)).sum::<u16>() / (op.width * op.height) as u16;
    pixels.iter().map(|&v| v as u16 > average).collect()
}

/// Provides difference hash (dHash) calculation.
pub struct DifferenceHash<'a> {
    op: &'a ImageOp,
}

impl<'a> DifferenceHash<'a> {
    /// Creates a new `DifferenceHasher` with default parameters.
    pub fn new() -> Self {
        DifferenceHash::default()
    }

    /// Creates a new `DifferenceHasher` with the specified parameters.
    pub fn with_op(op: &'a ImageOp) -> Self {
        DifferenceHash { op }
    }

    /// Calculates difference hash (dHash) of the image and returns as a hex string.
    pub fn hash(&self, image: &image::DynamicImage) -> String {
        let bits = difference_hash(image, self.op);
        let bytes = bits_to_bytes(&bits);
        bytes_to_hex(&bytes)
    }
}

impl Default for DifferenceHash<'_> {
    /// Creates a new `DifferenceHasher` with default parameters.
    fn default() -> Self {
        DifferenceHash {
            op: &ImageOp {
                width: 9,
                height: 8,
                filter: FilterType::Lanczos3,
            },
        }
    }
}

/// Calculates difference hash (dHash) of the image.
pub fn difference_hash(image: &image::DynamicImage, op: &ImageOp) -> Vec<bool> {
    let preprocessed = image
        .grayscale()
        .resize_exact(op.width as u32, op.height as u32, op.filter);
    let pixels = preprocessed.into_luma8().into_raw();
    let mut bits = vec![false; ((op.width - 1) * op.height) as usize];
    for y in 0..op.height {
        for x in 0..op.width - 1 {
            let offset_p = (y * op.width + x) as usize;
            let offset_b = (y * (op.width - 1) + x) as usize;
            bits[offset_b] = pixels[offset_p + 1] > pixels[offset_p];
        }
    }
    bits
}

fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = vec![0; (bits.len() + 7) / 8];
    for (i, bit) in bits.iter().enumerate() {
        if *bit {
            bytes[i / 8] |= 1 << (7 - (i % 8));
        }
    }
    bytes
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push_str(&format!("{:02x}", byte));
    }
    result
}
