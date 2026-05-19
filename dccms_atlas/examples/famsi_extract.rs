//! FAMSI PDF JPEG-stream extractor.
//!
//! The FAMSI Förstemann-Schele PDF (Dresden Codex chromolithograph
//! facsimile) embeds page images as raw `DCTDecode` (JPEG) streams.
//! We can extract them by scanning for the JPEG SOI marker
//! `FF D8 FF` and the EOI marker `FF D9` directly in the PDF bytes —
//! no PDF parser needed for this specific layout.
//!
//! Build/run:
//! ```text
//! cargo run --release --example famsi_extract
//! ```
//!
//! Outputs to `~/Agents/imports/famsi_dresden/extracted/`.

use std::fs;
use std::path::PathBuf;

fn home() -> PathBuf {
    let h = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .expect("home dir");
    PathBuf::from(h)
}

fn main() {
    let pdf_path = home()
        .join("Agents").join("imports").join("famsi_dresden")
        .join("famsi_pp13-24.pdf");
    let out_dir = home()
        .join("Agents").join("imports").join("famsi_dresden")
        .join("extracted");
    fs::create_dir_all(&out_dir).expect("mkdir");

    let bytes = fs::read(&pdf_path).expect("read FAMSI PDF");
    println!("FAMSI PDF: {} bytes loaded", bytes.len());

    // Scan for JPEG streams. SOI = FF D8 FF (next byte typically E0 / DB / EE).
    // EOI = FF D9 (last two bytes of the JPEG).
    let mut jpegs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        if bytes[i] == 0xFF && bytes[i + 1] == 0xD8 && bytes[i + 2] == 0xFF {
            // Found a SOI candidate. Scan forward for EOI.
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

    println!("Found {} JPEG stream(s).", jpegs.len());
    println!();
    for (idx, (start, end)) in jpegs.iter().enumerate() {
        let size = end - start;
        let out_path = out_dir.join(format!("page_{:02}.jpg", idx + 13));
        // FAMSI PDF is pages 13-24; index 0 → page 13, etc.
        // (If page-count doesn't line up to 12, we'll see in the listing.)
        fs::write(&out_path, &bytes[*start..*end]).expect("write JPEG");
        println!("  jpeg[{:>2}]: offset {:>9} bytes {:>9} → {}",
            idx, start, size, out_path.display());
    }
    println!();
    println!("Output dir: {}", out_dir.display());
}
