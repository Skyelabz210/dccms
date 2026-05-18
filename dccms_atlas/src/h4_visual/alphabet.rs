//! # H4 Visual Transducer — Core trait and semantic-role enum (NODE-VT01)
//!
//! This module defines the `GlyphAlphabet` trait and `SemanticRole` enum that
//! every visual-transducer implementor consumes. It is the *Object contract*
//! of the ns-continuum-bridge applied to a discrete-only setting: the domain
//! of any residue-space map MUST be a finite enumeration of typed glyphs.
//!
//! Pixels never reach this trait. The pixel→glyph segmenter (SEG nodes) is
//! a separate boundary layer that emits implementors of this trait. Once a
//! value is a `GlyphAlphabet`, it is by construction a member of a finite
//! alphabet and the residue map operates on it without continuum hazards.
//!
//! ## Operator Consistency obligation
//!
//! Every implementor MUST satisfy:
//!
//! ```text
//! g.address(&SAFE_BASIS) == cram_address(g.ordinal())
//! ```
//!
//! for every glyph `g`. This commutative-diagram closure is gated by
//! `NODE-CTR04` in [executioner_dag.md](../../../../executioner_dag.md).
//!
//! ## Basis-parameterized for arbitrary-precise scalability
//!
//! The trait's `address` method is generic in `const K: usize`. With `K=6`
//! the output is the canonical Safe Basis address. Extending to `K=7`
//! (basis = [2,3,5,7,11,13,17]) or `K=8` (+19) refines the addressable
//! range without altering the first 6 lanes — this is the load-bearing
//! scalability property, gated by `NODE-VT07`.

#![allow(dead_code)]

use dresden_codex::cram_address;

/// Semantic classification of a glyph's role in the codex.
///
/// Used by `NODE-FO01` to gate the fifth-operator coherence test:
/// `Navigation` glyphs MUST be active on lane p=11; `Content` glyphs MAY
/// have p=11 silent; `Boundary` glyphs control region transitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SemanticRole {
    /// Glyphs that encode position / coordinate / navigation
    /// (day-names, month-names, page-indices). MUST produce p=11-active
    /// CRAM addresses for non-zero ordinals.
    Navigation,
    /// Glyphs that encode payload / attribute / iconographic meaning
    /// (moon-sign, water-pot, weaving-shuttle, etc.). MAY have p=11
    /// silent; the Shadow16 product still applies.
    Content,
    /// Glyphs that demarcate configuration regions (eclipse-glyph,
    /// Wayeb-marker, count-terminator). Activate lane p=13 (boundary).
    Boundary,
}

/// The Object-contract trait for the H4 visual transducer.
///
/// `K` is the basis size: 6 for canonical Safe Basis, larger for the
/// arbitrary-precise lift gated by `NODE-VT07`.
pub trait GlyphAlphabet<const K: usize> {
    /// Ordinal value of this glyph in its alphabet. Maya base-20 for
    /// numerals, day-position for day-names, month-position for months,
    /// figure-index for iconographic glyphs.
    fn ordinal(&self) -> u64;

    /// Semantic role of this glyph in the codex configuration manifold.
    fn semantic_role(&self) -> SemanticRole;

    /// CRAM address of this glyph in the given basis.
    ///
    /// Default implementation: composes via `cram_address` of the
    /// ordinal, then extends to the given basis size. This default
    /// IS the Operator Consistency proof: by construction the
    /// implementor's address commutes with the canonical L1 operator.
    /// Implementors that override this method must add a test asserting
    /// they still commute (gated by `NODE-CTR04`).
    fn address(&self, basis: &[u64; K]) -> [u64; K] {
        let ord = self.ordinal();
        let mut addr = [0u64; K];
        for (i, &p) in basis.iter().enumerate() {
            addr[i] = ord % p;
        }
        addr
    }

}

/// Convenience free function: canonical Safe Basis address of a glyph.
///
/// Avoids the trait-self-reference ambiguity that arose with a default
/// method `safe_address`. Use as `safe_address(&glyph)` from any
/// implementor of `GlyphAlphabet<6>`.
pub fn safe_address<G: GlyphAlphabet<6>>(g: &G) -> [u64; 6] {
    cram_address(g.ordinal())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dresden_codex::SAFE_BASIS;

    /// Test scaffold implementor used to verify the trait defaults work.
    /// Real implementors live in sibling modules.
    struct ScaffoldGlyph(u64);
    impl GlyphAlphabet<6> for ScaffoldGlyph {
        fn ordinal(&self) -> u64 { self.0 }
        fn semantic_role(&self) -> SemanticRole { SemanticRole::Content }
    }

    #[test]
    fn default_address_commutes_with_cram_address() {
        // NODE-CTR04 (Operator Consistency) at the scaffold level.
        for ord in [0u64, 1, 5, 13, 19, 148, 177, 1448, 37960] {
            let g = ScaffoldGlyph(ord);
            assert_eq!(
                g.address(&SAFE_BASIS),
                cram_address(ord),
                "default trait address must equal cram_address(ordinal) for ord={}",
                ord
            );
        }
    }

    #[test]
    fn k7_lift_preserves_first_six_lanes() {
        // NODE-VT07 (BasisLift) at the scaffold level.
        const K7_BASIS: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
        struct G(u64);
        impl GlyphAlphabet<6> for G { fn ordinal(&self) -> u64 { self.0 } fn semantic_role(&self) -> SemanticRole { SemanticRole::Content } }
        impl GlyphAlphabet<7> for G { fn ordinal(&self) -> u64 { self.0 } fn semantic_role(&self) -> SemanticRole { SemanticRole::Content } }

        for ord in [0u64, 19, 148, 1448, 37960] {
            let g = G(ord);
            let k6 = <G as GlyphAlphabet<6>>::address(&g, &SAFE_BASIS);
            let k7 = <G as GlyphAlphabet<7>>::address(&g, &K7_BASIS);
            for i in 0..6 {
                assert_eq!(k6[i], k7[i],
                    "lane {} (basis prime {}) must agree between K=6 and K=7 for ord={}",
                    i, SAFE_BASIS[i], ord);
            }
            // The 17 lane carries the new residue.
            assert_eq!(k7[6], ord % 17);
        }
    }

    #[test]
    fn semantic_role_values_distinct() {
        assert_ne!(SemanticRole::Navigation, SemanticRole::Content);
        assert_ne!(SemanticRole::Navigation, SemanticRole::Boundary);
        assert_ne!(SemanticRole::Content, SemanticRole::Boundary);
    }
}
