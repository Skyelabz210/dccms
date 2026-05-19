//! Compare every SLUB page against its FAMSI counterpart (assuming
//! 1-to-1 index ordering — the open mapping question is reported but
//! not resolved here).
//!
//! Build/run:
//! ```text
//! cargo run --release --features slub --example compare_slub_famsi
//! ```

#![cfg(feature = "slub")]

use dccms_atlas::segmenter::comparison::compare_two_jpegs;
use std::path::PathBuf;

fn home() -> PathBuf {
    let h = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME")).expect("home");
    PathBuf::from(h)
}

fn slub_path(n: u32) -> PathBuf {
    home().join("Agents").join("imports").join("slub_dresden")
        .join(format!("page_{:08}.jpg", n))
}

fn famsi_path(n: u32) -> PathBuf {
    home().join("Agents").join("imports").join("famsi_dresden")
        .join("extracted").join(format!("page_{:02}.jpg", n))
}

fn main() {
    println!("══════════════════════════════════════════════════════════════════════");
    println!(" Cross-source comparison: SLUB photograph ↔ FAMSI chromolithograph");
    println!(" (assumes 1-to-1 page numbering; mapping uncertainty noted)");
    println!("══════════════════════════════════════════════════════════════════════");
    println!();
    println!(" page | SLUB comp | SLUB max  | FAMSI comp | FAMSI max | WWII?");
    println!(" -----+-----------+-----------+------------+-----------+------");
    let mut candidates: Vec<u32> = Vec::new();
    for n in 13..=24 {
        let sp = slub_path(n);
        let fp = famsi_path(n);
        if !sp.exists() || !fp.exists() { continue; }
        match compare_two_jpegs(&sp, &fp) {
            Ok(pc) => {
                let mark = if pc.wwii_damage_candidate { "★ YES" } else { "no" };
                println!("  {:>3} | {:>9} | {:>9} | {:>10} | {:>9} | {}",
                    n, pc.slub_components, pc.slub_max_area,
                    pc.famsi_components, pc.famsi_max_area, mark);
                if pc.wwii_damage_candidate { candidates.push(n); }
            }
            Err(e) => println!("  {:>3} | ERROR: {:?}", n, e),
        }
    }
    println!();
    println!(" WWII-damage candidates (SLUB blank-like, FAMSI content-bearing):");
    if candidates.is_empty() {
        println!("   (none)");
    } else {
        for n in &candidates {
            println!("   page {}: SLUB segments as blank but FAMSI shows content", n);
        }
    }
    println!();
    println!(" Open question: the 1-to-1 mapping assumption may not hold.");
    println!(" FAMSI extracted page_NN is in PDF-object order, NOT necessarily");
    println!(" Förstemann order. Visual cross-correlation or PDF page-tree");
    println!(" parsing required to confirm the mapping definitively.");
    println!("══════════════════════════════════════════════════════════════════════");
}
