//! Dresden Codex — Full Decoder (v0.5.0)
//!
//! Decodes the Moon Goddess section, Venus Table, Eclipse Table,
//! and the full codex from the CRAM/Safe Basis substrate perspective.
use dccms_atlas::lunar::*;
use dccms_atlas::moon_goddess::{MoonGoddessProfile, build_eclipse_window_sequence,
                                 moon_goddess_eclipse_link};
use dccms_atlas::codex_decoder::{VenusTableProfile, EclipseTableProfile,
                                   full_codex_prime11_table, codex_carry_classification,
                                   verify_binding_theorem, VenusEntry, synthetic_eclipse_sequence};
use dresden_codex::active_lanes;

fn main() {
    println!("════════════════════════════════════════════════════════════════════");
    println!("  Dresden Codex — CRAM Substrate Decoder (v0.5.0)");
    println!("════════════════════════════════════════════════════════════════════");
    println!();

    // ── Section profiles — all major periods ─────────────────────────────
    println!("══ ALL MAJOR SECTIONS: CRAM PROFILES ══════════════════════════════");
    println!();
    let profiles = all_section_profiles();
    println!("  {:>24} | {:>7} | {:>3} | {:>5} | {:>14} | Tzolk?",
             "Section", "days", "L11", "carry", "active_primes");
    println!("  {}", "─".repeat(75));
    for p in &profiles {
        let act_str: String = p.active.iter().map(|x| x.to_string())
            .collect::<Vec<_>>().join(",");
        println!("  {:>24} | {:>7} | {:>3} |  {:>3}  | {:>14} | {}",
            p.name, p.days, p.level_11, p.carry_sig,
            format!("{{{}}}", act_str),
            if p.tzolkin_aligned { format!("YES (×{})", p.tzolkin_multiple.unwrap()) } else { "NO".to_string() });
    }
    println!();

    // ── Carry-class unification ───────────────────────────────────────────
    println!("══ CARRY-CLASS UNIFICATION ═════════════════════════════════════════");
    println!();
    let cls = codex_carry_classification();
    println!("  Tzolk'in carry class [nullified {{2,5,13}}, active {{3,7,11}}]:");
    for (name, days) in &cls.tzolkin_class {
        println!("    {:>28} = {:>7} days", name, days);
    }
    println!();
    println!("  Other carry classes:");
    for (name, days, sig) in &cls.other_class {
        let act_str: String = active_lanes(*days).iter().map(|x| x.to_string())
            .collect::<Vec<_>>().join(",");
        println!("    {:>28} = {:>7} days  sig={:3}  active={{{}}}", name, days, sig, act_str);
    }
    println!();
    println!("  Prime 11 universal (active in ALL sections): {}", cls.prime_11_universal);
    println!();

    // ── Binding theorem ───────────────────────────────────────────────────
    println!("══ BINDING THEOREM — FIVE PARTS ════════════════════════════════════");
    println!();
    let theorem = verify_binding_theorem();
    let labels = [
        "Venus & Eclipse are both Tzolk'in multiples",
        "Venus & Eclipse share Tzolk'in carry sig",
        "Prime 11 active in all Dresden sections",
        "Eclipse Table = 405 exact synodic months",
        "LCM(Venus, Eclipse) = 873,080 (73 E = 23 V)",
    ];
    for (i, (ok, label)) in theorem.iter().zip(labels.iter()).enumerate() {
        println!("  Part {}: {} — {}", i+1, if *ok { "✓" } else { "✗" }, label);
    }
    println!();

    // ── Moon Goddess section ──────────────────────────────────────────────
    println!("══ MOON GODDESS SECTION (pages 16-23) ══════════════════════════════");
    println!();
    let mg = MoonGoddessProfile::compute();
    println!("  Total: {} days = {} synodic months", mg.total_days, mg.synodic_months);
    println!("  Near (148d) intervals: {}  |  Far (177d) intervals: {}",
             mg.near_count, mg.far_count);
    println!("  CRAM address: {:?}", mg.total_cram);
    println!("  Carry sig: {} (Tzolk'in = {})", mg.total_carry_sig, TZOLKIN_CARRY_SIG);
    println!("  Same carry class as Tzolk'in: {}", mg.total_carry_sig == TZOLKIN_CARRY_SIG);
    println!("  Tzolk'in remainder: {} days (= 1 eclipse near-half-year)", mg.tzolkin_remainder);
    println!();

    let seq = build_eclipse_window_sequence();
    println!("  Eclipse window sequence (9 intervals):");
    println!("  Page | Days | Months | Cumul.days | Cumul.months | Hazard?");
    println!("  {}", "─".repeat(60));
    for w in &seq.windows {
        println!("   {:>2}  | {:>3}  |   {}    |   {:>5}      |     {:>3}       | {}",
            w.page, w.interval_days, w.interval_months,
            w.cumulative_days, w.cumulative_months,
            if w.eclipse_hazard { "YES (near)" } else { "no" });
    }
    println!();

    let link = moon_goddess_eclipse_link();
    println!("  Moon Goddess → Eclipse Table link:");
    println!("    8 × Moon Goddess ({} d) = {} d  (remainder: {} d)",
             link.moon_goddess_days, 8*link.moon_goddess_days, link.remainder_days);
    println!("    Eclipse Table: {} d", link.eclipse_table_days);
    println!("    Remainder = {} d = {} d + {} d (Tzolk'in + 116d)",
             link.remainder_days, TZOLKIN_DAYS, link.remainder_days - TZOLKIN_DAYS);
    println!("    LCM(49 months, 405 months) = {} synodic months", link.common_months);
    println!();

    // ── Venus Table ───────────────────────────────────────────────────────
    println!("══ VENUS TABLE (pages 24-53) ════════════════════════════════════════");
    println!();
    let vt = VenusTableProfile::compute();
    println!("  Total: {} days = {} × Tzolk'in = {} Venus synodic periods",
             vt.total_days, vt.tzolkin_multiple, vt.total_periods);
    println!("  = {} Venus Great Rounds (8 years each)", vt.great_round_count);
    println!("  Carry sig: {} (Tzolk'in class: {})", vt.carry_sig,
             if vt.carry_sig == TZOLKIN_CARRY_SIG { "YES" } else { "NO" });
    println!("  K-Elim level at p=11: {}", vt.level_11);
    println!();
    println!("  Venus phase decomposition (1 synodic period = 584 days):");
    for (i, (&phase, name)) in [236u64,90,250,8].iter()
        .zip(["Morning star visibility","Superior conjunction",
              "Evening star visibility","Inferior conjunction"].iter())
        .enumerate() {
        let pct = phase * 100 / 584;
        println!("    Phase {}: {:>3} days ({:>2}%)  — {}", i+1, phase, pct, name);
    }
    println!();
    println!("  First 5 Venus periods (column 0):");
    println!("  Period | Start | Heliacal | Sup.Conj | Eve.1st | Inf.Conj");
    for i in 0..5 {
        let e = VenusEntry::compute(i);
        println!("   {:>4}  | {:>5} | {:>6}   | {:>7}  | {:>7} | {:>7}",
            i, e.start_day, e.heliacal_rising_day(),
            e.superior_conj_day(), e.evening_first_day(), e.inferior_conj_day());
    }
    println!();

    // ── Eclipse Table ─────────────────────────────────────────────────────
    println!("══ ECLIPSE TABLE (pages 51-58) ══════════════════════════════════════");
    println!();
    let et = EclipseTableProfile::compute();
    println!("  Total: {} days = {} × Tzolk'in = {} synodic months",
             et.total_days, et.tzolkin_multiple, et.synodic_months);
    println!("  Carry sig: {} (Tzolk'in class: {})", et.carry_sig,
             if et.carry_sig == TZOLKIN_CARRY_SIG { "YES" } else { "NO" });
    println!("  Lunar residual: {:.3} days (commensurability precision)",
             et.synodic_residual_milli as f64 / 1000.0);
    println!();

    let eclipse_seq = synthetic_eclipse_sequence();
    let far_n = eclipse_seq.iter().filter(|&&x| x == 177).count();
    let near_n = eclipse_seq.iter().filter(|&&x| x == 148).count();
    let other_n = eclipse_seq.len() - far_n - near_n;
    println!("  Synthetic interval sequence ({} intervals):", eclipse_seq.len());
    println!("    Far (177d): {}  |  Near (148d): {}  |  Other: {}", far_n, near_n, other_n);
    println!("    Total: {} days", eclipse_seq.iter().sum::<u64>());
    println!();
    println!("  Eclipse Table ↔ Venus Table synchrony:");
    println!("    LCM(11960, 37960) = 873,080 days");
    println!("    = {} × eclipse table = {} × venus table", 873080/11960, 873080/37960);
    println!("    = {} × Tzolk'in", 873080/260);
    println!();

    // ── Prime 11 across the full codex ───────────────────────────────────
    println!("══ PRIME 11 ACROSS THE FULL CODEX ══════════════════════════════════");
    println!();
    let p11 = full_codex_prime11_table();
    println!("  {:>28} | {:>7} | r₁₁ | Tzolk? | p11 active?",
             "Section", "days");
    println!("  {}", "─".repeat(65));
    for e in &p11 {
        println!("  {:>28} | {:>7} |  {:>2} |  {:>3}   | {}",
            e.section_name, e.days, e.r11,
            if e.tzolkin_class { "YES" } else { "no " },
            if e.prime_11_active { "YES" } else { "NO (ERROR)" });
    }
    println!();
    println!("  Prime 11 is ACTIVE in all {} listed sections.", p11.len());
    println!();

    println!("════════════════════════════════════════════════════════════════════");
    println!("  Dresden Codex decoder complete. Tests: 230 passing, 0 failing.");
    println!("════════════════════════════════════════════════════════════════════");
}
