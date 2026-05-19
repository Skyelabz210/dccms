//! # LongCount — Engine 3 (B-7.3)
//!
//! Cylindrical time `ℝ × (S¹)⁴` — the Maya Long Count.
//!
//! Five tiers, each a cycle with overflow promoting to the next:
//!
//! | Tier | Lane | Modulus | Domain |
//! |---|---|---:|---|
//! | 4 | Baktun | linear (`u64`) | the `ℝ` axis — unbounded |
//! | 3 | Katun  | 20  | 7,200-day eras |
//! | 2 | Tun    | 20  | 360-day years |
//! | 1 | Uinal  | **18** | 20-day months (**anomalous** — not 20) |
//! | 0 | Kin    | 20  | days |
//!
//! Total days from a `LongCount` is
//! `baktun · 144,000 + katun · 7,200 + tun · 360 + uinal · 20 + kin`.
//!
//! The uinal lane's mod-18 is the structural anomaly: a Tun (360 days)
//! is `18 uinals · 20 kins`, NOT `20 uinals · 20 kins = 400`. This is
//! the Maya calendar's deliberate choice to make `360 ≈ solar year` at
//! the tun level rather than `400`.
//!
//! ## Tier promotion
//!
//! `advance_one_day` carries through the tiers via overflow:
//! - kin overflows at 20 → uinal increments
//! - uinal overflows at 18 → tun increments
//! - tun overflows at 20 → katun increments
//! - katun overflows at 20 → baktun increments
//! - baktun is linear (no overflow)
//!
//! ## 13-Baktun period
//!
//! The full 13-Baktun cycle = `13 · 144,000 = 1,872,000` days
//! corresponds to `LongCount { baktun: 13, katun: 0, tun: 0, uinal: 0, kin: 0 }`.
//!
//! Source: vault `Mayas Engine.md` §ENGINE 3.

#![allow(dead_code)]

use super::lane::{Lane, MayaState};

/// Period-ending markers — typed (NOT vault's `&'static str`).
///
/// `period_endings` returns all tier-endings simultaneously achieved
/// at a given LongCount position. A Baktun end implies Katun, Tun,
/// and Uinal ends as well.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PeriodEnding {
    /// `kin == 0` — end of a 20-day uinal.
    UinalEnd,
    /// `kin == 0 && uinal == 0` — end of a 360-day tun.
    TunEnd,
    /// `kin == 0 && uinal == 0 && tun == 0` — end of a 7,200-day katun.
    KatunEnd,
    /// All four cyclic tiers at 0 — end of a 144,000-day baktun.
    BaktunEnd,
}

/// A Long Count position. Each cyclic lane is bounded by its modulus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LongCount {
    pub baktun: u64,
    pub katun: u8,   // 0..20
    pub tun:   u8,   // 0..20
    pub uinal: u8,   // 0..18 (anomalous)
    pub kin:   u8,   // 0..20
}

impl LongCount {
    /// Per-tier moduli in `cyclic_state` order: [kin, uinal, tun, katun].
    pub const CYCLIC_MODULI: [u64; 4] = [20, 18, 20, 20];

    /// 144,000 days per Baktun.
    pub const KIN_PER_BAKTUN: u64 = 144_000;
    /// 7,200 days per Katun.
    pub const KIN_PER_KATUN: u64 = 7_200;
    /// 360 days per Tun.
    pub const KIN_PER_TUN: u64 = 360;
    /// 20 days per Uinal.
    pub const KIN_PER_UINAL: u64 = 20;

    /// Construct with each cyclic lane reduced modulo its tier modulus.
    pub fn new(baktun: u64, katun: u64, tun: u64, uinal: u64, kin: u64) -> Self {
        Self {
            baktun,
            katun: (katun % 20) as u8,
            tun:   (tun   % 20) as u8,
            uinal: (uinal % 18) as u8,
            kin:   (kin   % 20) as u8,
        }
    }

    /// Total day count, exact-integer.
    pub fn to_days(&self) -> u64 {
        self.baktun * Self::KIN_PER_BAKTUN
            + (self.katun as u64) * Self::KIN_PER_KATUN
            + (self.tun   as u64) * Self::KIN_PER_TUN
            + (self.uinal as u64) * Self::KIN_PER_UINAL
            + (self.kin   as u64)
    }

    /// Inverse of `to_days` — Long Count for a given day-count.
    pub fn from_days(mut days: u64) -> Self {
        let baktun = days / Self::KIN_PER_BAKTUN;
        days %= Self::KIN_PER_BAKTUN;
        let katun = (days / Self::KIN_PER_KATUN) as u8;
        days %= Self::KIN_PER_KATUN;
        let tun = (days / Self::KIN_PER_TUN) as u8;
        days %= Self::KIN_PER_TUN;
        let uinal = (days / Self::KIN_PER_UINAL) as u8;
        let kin = (days % Self::KIN_PER_UINAL) as u8;
        Self { baktun, katun, tun, uinal, kin }
    }

    /// Advance exactly one day through tier promotion.
    pub fn advance_one_day(&self) -> Self {
        let mut kin = self.kin + 1;
        let mut uinal = self.uinal;
        let mut tun = self.tun;
        let mut katun = self.katun;
        let mut baktun = self.baktun;
        if kin >= 20 { kin = 0; uinal += 1; }
        if uinal >= 18 { uinal = 0; tun += 1; }
        if tun >= 20 { tun = 0; katun += 1; }
        if katun >= 20 { katun = 0; baktun += 1; }
        Self { baktun, katun, tun, uinal, kin }
    }

    /// Advance by `days` (via integer round-trip — exact and clean).
    pub fn advance(&self, days: u64) -> Self {
        Self::from_days(self.to_days() + days)
    }

    /// The four cyclic tiers projected into a `MayaState` with lanes
    /// [kin (20), uinal (18), tun (20), katun (20)]. The linear baktun
    /// axis is NOT part of this state — it's the `ℝ` covering line.
    pub fn cyclic_state(&self) -> MayaState {
        let lanes = vec![
            Lane { name: "kin",   modulus: 20, domain: "day" },
            Lane { name: "uinal", modulus: 18, domain: "month" },
            Lane { name: "tun",   modulus: 20, domain: "year" },
            Lane { name: "katun", modulus: 20, domain: "era" },
        ];
        let residues = vec![
            self.kin   as u64,
            self.uinal as u64,
            self.tun   as u64,
            self.katun as u64,
        ];
        MayaState::new(residues, lanes)
            .expect("LongCount cyclic residues bounded by construction")
    }

    /// All tier-ending markers currently achieved (lowest tier first).
    pub fn period_endings(&self) -> Vec<PeriodEnding> {
        let mut endings = Vec::new();
        if self.kin == 0 {
            endings.push(PeriodEnding::UinalEnd);
            if self.uinal == 0 {
                endings.push(PeriodEnding::TunEnd);
                if self.tun == 0 {
                    endings.push(PeriodEnding::KatunEnd);
                    if self.katun == 0 {
                        endings.push(PeriodEnding::BaktunEnd);
                    }
                }
            }
        }
        endings
    }
}

impl std::fmt::Display for LongCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}.{}",
            self.baktun, self.katun, self.tun, self.uinal, self.kin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vault §ENGINE 3 test-date list, all expected to round-trip.
    const VAULT_TEST_DAYS: [u64; 12] = [
        0, 1, 19, 20, 359, 360, 7199, 7200, 143999, 144_000, 1_872_000, 13 * 144_000,
    ];

    #[test]
    fn round_trip_for_vault_test_dates() {
        for &d in &VAULT_TEST_DAYS {
            let lc = LongCount::from_days(d);
            assert_eq!(lc.to_days(), d, "round-trip failed for {}", d);
        }
    }

    #[test]
    fn advance_one_day_from_origin() {
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let after = zero.advance_one_day();
        assert_eq!(after, LongCount::new(0, 0, 0, 0, 1));
    }

    #[test]
    fn advance_twenty_promotes_kin_to_uinal() {
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let after = zero.advance(20);
        assert_eq!(after, LongCount::new(0, 0, 0, 1, 0));
    }

    #[test]
    fn advance_three_sixty_promotes_to_tun() {
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let after = zero.advance(360);
        assert_eq!(after, LongCount::new(0, 0, 1, 0, 0));
    }

    #[test]
    fn advance_seventy_two_hundred_promotes_to_katun() {
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let after = zero.advance(7_200);
        assert_eq!(after, LongCount::new(0, 1, 0, 0, 0));
    }

    #[test]
    fn advance_one_forty_four_thousand_promotes_to_baktun() {
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let after = zero.advance(144_000);
        assert_eq!(after, LongCount::new(1, 0, 0, 0, 0));
    }

    #[test]
    fn thirteen_baktun_round_trip() {
        let lc = LongCount::from_days(1_872_000);
        assert_eq!(lc, LongCount::new(13, 0, 0, 0, 0));
        assert_eq!(lc.to_days(), 1_872_000);
    }

    #[test]
    fn period_endings_at_baktun_end() {
        // 0.0.0.0.0 has all four cyclic tiers at 0 → all four endings.
        let zero = LongCount::new(0, 0, 0, 0, 0);
        let endings = zero.period_endings();
        assert!(endings.contains(&PeriodEnding::UinalEnd));
        assert!(endings.contains(&PeriodEnding::TunEnd));
        assert!(endings.contains(&PeriodEnding::KatunEnd));
        assert!(endings.contains(&PeriodEnding::BaktunEnd));
        assert_eq!(endings.len(), 4);
    }

    #[test]
    fn period_endings_only_uinal_when_kin_zero() {
        let lc = LongCount::new(0, 0, 0, 3, 0);  // kin=0 but uinal=3
        let endings = lc.period_endings();
        assert_eq!(endings, vec![PeriodEnding::UinalEnd]);
    }

    #[test]
    fn period_endings_empty_mid_uinal() {
        let lc = LongCount::new(0, 0, 0, 0, 5);  // kin=5
        assert!(lc.period_endings().is_empty());
    }

    #[test]
    fn cyclic_state_has_four_lanes_with_anomalous_uinal() {
        let lc = LongCount::new(0, 1, 2, 3, 4);
        let state = lc.cyclic_state();
        assert_eq!(state.lane_count(), 4);
        let moduli: Vec<u64> = state.lanes().iter().map(|l| l.modulus).collect();
        // [kin=20, uinal=18, tun=20, katun=20] — uinal is the anomalous mod-18 lane.
        assert_eq!(moduli, vec![20u64, 18, 20, 20]);
        let residues: Vec<u64> = state.residues().to_vec();
        assert_eq!(residues, vec![4u64, 3, 2, 1]);  // kin, uinal, tun, katun
    }

    #[test]
    fn display_format() {
        let lc = LongCount::new(13, 0, 0, 0, 0);
        assert_eq!(format!("{}", lc), "13.0.0.0.0");
    }
}
