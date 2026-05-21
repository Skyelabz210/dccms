//! FAMSI PDF JPEG-stream extractor — v0.9.5 N18: all 6 PDFs.
//!
//! The FAMSI Förstemann-Schele PDFs (Dresden Codex chromolithograph
//! facsimile) embed page images as raw `DCTDecode` (JPEG) streams.
//! We can extract them by scanning for the JPEG SOI marker `FF D8 FF`
//! and the EOI marker `FF D9` directly in the PDF bytes — no PDF
//! parser needed for this specific layout.
//!
//! v0.9.5 extends from a single PDF (pp13-24) to all 6:
//!     pp01-12 / pp13-24 / pp25-35 / pp36-45 / pp46-59 / pp60-74
//!
//! Each PDF's JPEGs are emitted to `~/Agents/imports/famsi_dresden/extracted/`
//! as `page_NN.jpg` where `NN` is the Förstemann page number. The
//! extraction-order ↔ Förstemann-order mapping was empirically verified
//! 1-to-1 at 98% similarity for pp13-24 (`calibrate.rs` Job 2). The same
//! assumption is applied uniformly to the other 5 PDFs.
//!
//! Build/run:
//! ```text
//! cargo run --release --example famsi_extract
//! ```

use dccms_atlas::paths::{famsi_dir, famsi_extracted_dir, FAMSI_SOURCE_PDFS as PDFS};
use std::fs;

/// Byte-scan a PDF buffer for JPEG (SOI..EOI) streams.
/// Returns `(start, end)` byte offsets, inclusive of SOI and the byte
/// after EOI.
fn scan_jpeg_streams(bytes: &[u8]) -> Vec<(usize, usize)> {
    let mut jpegs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        if bytes[i] == 0xFF && bytes[i + 1] == 0xD8 && bytes[i + 2] == 0xFF {
            let mut j = i + 3;
            while j + 1 < bytes.len() {
                if bytes[j] == 0xFF && bytes[j + 1] == 0xD9 {
                    jpegs.push((i, j + 2));
                    i = j + 2;
                    break;
                }
                j += 1;
            }
            if j + 1 >= bytes.len() { break; }
        } else {
            i += 1;
        }
    }
    jpegs
}

fn main() {
    let dir = famsi_dir();
    let out_dir = famsi_extracted_dir();
    fs::create_dir_all(&out_dir).expect("mkdir");

    println!("══════════════════════════════════════════════════════════════════════");
    println!(" FAMSI byte-scan extractor — full-codex sweep (6 PDFs)");
    println!("══════════════════════════════════════════════════════════════════════");

    let mut total_extracted: u32 = 0;
    let mut total_skipped: u32 = 0;
    let mut total_missing_pdfs: u32 = 0;

    for &(filename, first, last) in PDFS.iter() {
        let pdf_path = dir.join(filename);
        let expected_count = last - first + 1;
        if !pdf_path.exists() {
            println!();
            println!(" {} — NOT PRESENT (skipping)", filename);
            total_missing_pdfs += 1;
            continue;
        }
        let bytes = fs::read(&pdf_path).expect("read FAMSI PDF");
        let jpegs = scan_jpeg_streams(&bytes);
        println!();
        println!(" {} ({} bytes)", filename, bytes.len());
        println!("   pages {}-{} expected, {} JPEG streams found",
            first, last, jpegs.len());
        if jpegs.len() as u32 != expected_count {
            println!("   ⚠ count mismatch — applying 1-to-1 mapping to min(found, expected)");
        }
        let n = (jpegs.len() as u32).min(expected_count);
        for (idx, &(start, end)) in jpegs.iter().take(n as usize).enumerate() {
            let page = first + idx as u32;
            let size = end - start;
            let out_path = out_dir.join(format!("page_{:02}.jpg", page));
            // Don't clobber pages 13-24 already on disk if extraction
            // gives the same byte content (v0.9.0 byte-scan was
            // identical to this one); skip only when on-disk size
            // matches exactly.
            let skip = if out_path.exists() {
                fs::metadata(&out_path).map(|m| m.len() as usize == size).unwrap_or(false)
            } else { false };
            if skip {
                total_skipped += 1;
                continue;
            }
            fs::write(&out_path, &bytes[start..end]).expect("write JPEG");
            total_extracted += 1;
            println!("   jpeg[{:>2}] @ {:>9}  {:>8} bytes  →  page_{:02}.jpg",
                idx, start, size, page);
        }
    }

    println!();
    println!("──────────────────────────────────────────────────────────────────────");
    println!(" extracted: {}    skipped (unchanged): {}    missing PDFs: {}",
        total_extracted, total_skipped, total_missing_pdfs);
    println!(" output dir: {}", out_dir.display());
    println!("══════════════════════════════════════════════════════════════════════");
}
