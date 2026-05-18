//! # Codex Events — Encoding What the Codex Records
//!
//! The codex tracks specific kinds of events: Venus heliacal risings,
//! eclipse predictions, ritual dates, agricultural milestones,
//! Calendar Round renewals, etc. Each event is recorded as a Long
//! Count timestamp plus an interpretation tag.
//!
//! For DCCMS, each event becomes a *data point* in the address space
//! of the four-head Hydra. Events with the same Hydra address belong
//! to the same configuration. Events that cluster reveal the codex's
//! preferred configurations.
//!
//! ## Event Representation
//!
//! Every event carries:
//! - A `days_since_epoch` integer (Long Count interpretation)
//! - An `event_kind` tag (what was recorded)
//! - The CRAM address (residue tuple on Safe Basis)
//! - The four-head signature (which configuration the event lives in)
//!
//! Provenance: this module encodes events from the documented Maya
//! astronomical record. It does not re-derive astronomy from the
//! codex. It uses dates from the Dresden Codex tables as published
//! events to test against.

use crate::heads::FourCalendarHydra;
use dresden_codex::cram_address;
use std::collections::HashMap;

/// The kind of event recorded in the codex.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CodexEventKind {
    /// Venus heliacal rising (start of morning-star phase).
    VenusHeliacalRising,
    /// Venus inferior conjunction (end of morning-star, beginning of invisibility).
    VenusInferiorConjunction,
    /// Venus superior conjunction (end of evening-star, beginning of invisibility).
    VenusSuperiorConjunction,
    /// Venus evening-star first visibility.
    VenusEveningFirstVisibility,
    /// Lunar eclipse prediction (from the 11,960-day eclipse table).
    LunarEclipsePrediction,
    /// Solar eclipse prediction.
    SolarEclipsePrediction,
    /// Tzolk'in cycle completion (day 260, day 520, ...).
    TzolkinCycleComplete,
    /// Haab year completion (the New Year, end of Wayeb).
    HaabYearComplete,
    /// Calendar Round completion (18,980 days = 52 Haab years).
    CalendarRoundComplete,
    /// 819-day Planetary Council station.
    PlanetaryCouncilStation,
    /// Long Count Katun ending (every 7,200 days).
    KatunEnding,
    /// Long Count Baktun ending (every 144,000 days).
    BaktunEnding,
    /// Mars heliacal event.
    MarsHeliacalEvent,
    /// Generic ritual date from the codex almanac sections.
    RitualDate,
    /// Custom event with annotation.
    Custom(&'static str),
}

impl CodexEventKind {
    /// Short label for logging/printing.
    pub fn label(&self) -> &'static str {
        match self {
            CodexEventKind::VenusHeliacalRising => "VenusHelRise",
            CodexEventKind::VenusInferiorConjunction => "VenusInfConj",
            CodexEventKind::VenusSuperiorConjunction => "VenusSupConj",
            CodexEventKind::VenusEveningFirstVisibility => "VenusEvFirst",
            CodexEventKind::LunarEclipsePrediction => "LunarEclipse",
            CodexEventKind::SolarEclipsePrediction => "SolarEclipse",
            CodexEventKind::TzolkinCycleComplete => "TzolkinCycle",
            CodexEventKind::HaabYearComplete => "HaabYear",
            CodexEventKind::CalendarRoundComplete => "CalendarRound",
            CodexEventKind::PlanetaryCouncilStation => "Council819",
            CodexEventKind::KatunEnding => "Katun",
            CodexEventKind::BaktunEnding => "Baktun",
            CodexEventKind::MarsHeliacalEvent => "MarsHelEv",
            CodexEventKind::RitualDate => "Ritual",
            CodexEventKind::Custom(s) => s,
        }
    }
}

/// A single codex event.
#[derive(Clone, Debug)]
pub struct CodexEvent {
    /// Days since the Long Count epoch (13.0.0.0.0 = day 0 by convention).
    pub days_since_epoch: u64,
    /// What kind of event this is.
    pub kind: CodexEventKind,
    /// The CRAM address (residue tuple on the Safe Basis).
    pub address: [u64; 6],
    /// Notes / source citation.
    pub note: String,
}

impl CodexEvent {
    pub fn new(days_since_epoch: u64, kind: CodexEventKind, note: impl Into<String>) -> Self {
        CodexEvent {
            days_since_epoch,
            kind,
            address: cram_address(days_since_epoch),
            note: note.into(),
        }
    }
}

/// A collection of codex events.
#[derive(Clone, Debug, Default)]
pub struct EventSet {
    pub events: Vec<CodexEvent>,
}

impl EventSet {
    pub fn new() -> Self {
        EventSet::default()
    }

    pub fn add(&mut self, event: CodexEvent) {
        self.events.push(event);
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Generate the canonical set of Tzolk'in cycle completions over
    /// the operational time range. One event per Tzolk'in completion.
    pub fn tzolkin_completions(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut day = (start_day / 260 + 1) * 260;
        while day <= end_day {
            set.add(CodexEvent::new(
                day,
                CodexEventKind::TzolkinCycleComplete,
                format!("Tzolk'in completion #{}", day / 260),
            ));
            day += 260;
        }
        set
    }

    /// Generate Haab year completions.
    pub fn haab_completions(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut day = (start_day / 365 + 1) * 365;
        while day <= end_day {
            set.add(CodexEvent::new(
                day,
                CodexEventKind::HaabYearComplete,
                format!("Haab year completion #{}", day / 365),
            ));
            day += 365;
        }
        set
    }

    /// Generate Venus synodic events: each 584-day cycle has four
    /// canonical phase transitions at offsets 0, 236, 326, 576.
    pub fn venus_synodic_events(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut cycle_start = (start_day / 584) * 584;
        loop {
            if cycle_start > end_day { break; }
            let phases = [
                (0u64, CodexEventKind::VenusHeliacalRising, "morning-star begins"),
                (236, CodexEventKind::VenusSuperiorConjunction, "morning-star ends"),
                (326, CodexEventKind::VenusEveningFirstVisibility, "evening-star begins"),
                (576, CodexEventKind::VenusInferiorConjunction, "evening-star ends"),
            ];
            for (offset, kind, note) in phases {
                let day = cycle_start + offset;
                if day >= start_day && day <= end_day {
                    set.add(CodexEvent::new(day, kind, note));
                }
            }
            cycle_start += 584;
        }
        set
    }

    /// Generate 819-day Planetary Council stations.
    pub fn council_819_stations(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut day = (start_day / 819 + 1) * 819;
        while day <= end_day {
            set.add(CodexEvent::new(
                day,
                CodexEventKind::PlanetaryCouncilStation,
                format!("819-day station #{}", day / 819),
            ));
            day += 819;
        }
        set
    }

    /// Generate Katun endings (every 7,200 days).
    pub fn katun_endings(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut day = (start_day / 7200 + 1) * 7200;
        while day <= end_day {
            set.add(CodexEvent::new(
                day,
                CodexEventKind::KatunEnding,
                format!("Katun ending #{}", day / 7200),
            ));
            day += 7200;
        }
        set
    }

    /// Generate Calendar Round renewals (every 18,980 days = 52 Haab years).
    pub fn calendar_round_renewals(start_day: u64, end_day: u64) -> Self {
        let mut set = EventSet::new();
        let mut day = (start_day / 18980 + 1) * 18980;
        while day <= end_day {
            set.add(CodexEvent::new(
                day,
                CodexEventKind::CalendarRoundComplete,
                format!("CR renewal #{}", day / 18980),
            ));
            day += 18980;
        }
        set
    }

    /// Compose a canonical event set covering all the major cycle types
    /// for a given operational time range.
    pub fn canonical_corpus(start_day: u64, end_day: u64) -> Self {
        let mut combined = EventSet::new();
        combined.events.extend(Self::tzolkin_completions(start_day, end_day).events);
        combined.events.extend(Self::haab_completions(start_day, end_day).events);
        combined.events.extend(Self::venus_synodic_events(start_day, end_day).events);
        combined.events.extend(Self::council_819_stations(start_day, end_day).events);
        combined.events.extend(Self::katun_endings(start_day, end_day).events);
        combined.events.extend(Self::calendar_round_renewals(start_day, end_day).events);
        combined
    }

    /// Group events by their four-head address.
    /// Returns a map from flat-address (u32) to event indices.
    pub fn group_by_four_head_address(&self, hydra: &FourCalendarHydra) -> HashMap<u32, Vec<usize>> {
        let mut groups: HashMap<u32, Vec<usize>> = HashMap::new();
        for (idx, event) in self.events.iter().enumerate() {
            let addr = hydra.flat_address(event.days_since_epoch);
            groups.entry(addr).or_default().push(idx);
        }
        groups
    }

    /// Count distinct four-head addresses occupied by this event set.
    pub fn distinct_addresses(&self, hydra: &FourCalendarHydra) -> usize {
        self.group_by_four_head_address(hydra).len()
    }

    /// Compute occupancy histogram: how many addresses have how many events.
    pub fn occupancy_histogram(&self, hydra: &FourCalendarHydra) -> HashMap<usize, usize> {
        let groups = self.group_by_four_head_address(hydra);
        let mut hist: HashMap<usize, usize> = HashMap::new();
        for (_, events) in groups {
            *hist.entry(events.len()).or_insert(0) += 1;
        }
        hist
    }

    /// Filter events by kind.
    pub fn filter_by_kind(&self, kind: &CodexEventKind) -> Vec<&CodexEvent> {
        self.events.iter()
            .filter(|e| &e.kind == kind)
            .collect()
    }

    /// Distinct kinds present in the event set.
    pub fn distinct_kinds(&self) -> Vec<CodexEventKind> {
        let mut kinds: Vec<CodexEventKind> = Vec::new();
        for event in &self.events {
            if !kinds.contains(&event.kind) {
                kinds.push(event.kind.clone());
            }
        }
        kinds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tzolkin_completions_have_zero_residue_mod_260_components() {
        let set = EventSet::tzolkin_completions(0, 2600);
        assert_eq!(set.len(), 10);
        for event in &set.events {
            // 260 = 2² × 5 × 13 — these residues should be 0
            assert_eq!(event.address[0], 0, "residue mod 2 should be 0");
            assert_eq!(event.address[2], 0, "residue mod 5 should be 0");
            assert_eq!(event.address[5], 0, "residue mod 13 should be 0");
        }
    }

    #[test]
    fn haab_completions_have_zero_residue_mod_5() {
        let set = EventSet::haab_completions(0, 3650);
        assert_eq!(set.len(), 10);
        for event in &set.events {
            // 365 = 5 × 73 — residue mod 5 should be 0
            assert_eq!(event.address[2], 0, "residue mod 5 should be 0");
        }
    }

    #[test]
    fn venus_synodic_events_have_four_per_cycle() {
        // A single Venus synodic cycle covers days [0, 583]. Within that
        // range the four canonical phases occur at offsets 0, 236, 326, 576.
        let set = EventSet::venus_synodic_events(0, 583);
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn venus_synodic_events_include_phase_at_cycle_start() {
        // Asking for range [0, 584] inclusive includes day 584 which is
        // the start of cycle 1 (also at phase-0 offset). So we get 5 events
        // for [0, 584]: cycle 0 has all four phases + cycle 1 phase 0.
        let set = EventSet::venus_synodic_events(0, 584);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn council_819_events_align_with_factorization() {
        let set = EventSet::council_819_stations(0, 8190);
        assert_eq!(set.len(), 10);
        for event in &set.events {
            // 819 = 3² × 7 × 13 — these residues should be 0
            assert_eq!(event.address[1], 0, "residue mod 3 should be 0");
            assert_eq!(event.address[3], 0, "residue mod 7 should be 0");
            assert_eq!(event.address[5], 0, "residue mod 13 should be 0");
        }
    }

    #[test]
    fn canonical_corpus_grows_with_range() {
        let small = EventSet::canonical_corpus(0, 10_000);
        let large = EventSet::canonical_corpus(0, 100_000);
        assert!(large.len() > small.len());
    }

    #[test]
    fn corpus_distinct_addresses_under_four_head() {
        let hydra = FourCalendarHydra::canonical();
        let corpus = EventSet::canonical_corpus(0, 50_000);
        let distinct = corpus.distinct_addresses(&hydra);
        // Should occupy multiple distinct four-head addresses
        assert!(distinct > 1);
        // Should not exceed total events (no degenerate hashing)
        assert!(distinct <= corpus.len());
    }

    #[test]
    fn katun_endings_have_address_pattern() {
        let set = EventSet::katun_endings(0, 72_000);
        assert_eq!(set.len(), 10);
        for event in &set.events {
            // 7200 = 2^5 × 3² × 5² — nullifies {2, 3, 5}
            assert_eq!(event.address[0], 0);
            assert_eq!(event.address[1], 0);
            assert_eq!(event.address[2], 0);
        }
    }

    #[test]
    fn calendar_round_events_align() {
        let set = EventSet::calendar_round_renewals(0, 189_800);
        assert_eq!(set.len(), 10);
        for event in &set.events {
            // 18,980 = 2² × 5 × 13 × 73 — nullifies {2, 5, 13}
            assert_eq!(event.address[0], 0);
            assert_eq!(event.address[2], 0);
            assert_eq!(event.address[5], 0);
        }
    }
}
