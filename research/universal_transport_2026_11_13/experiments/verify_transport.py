#!/usr/bin/env python3
"""Exhaustively verify the session's two finite transport constructors.

The decoders accept only bounded readings.  The loop variable ``x`` is used by
this test harness to generate authentic inputs and expected answers; it is never
passed to either decoder.
"""

from __future__ import annotations

import argparse
import json
import math
from datetime import datetime, timezone
from pathlib import Path


def require_reading(name: str, value: int, modulus: int) -> None:
    if isinstance(value, bool) or not isinstance(value, int):
        raise TypeError(f"{name} must be an integer")
    if not 0 <= value < modulus:
        raise ValueError(f"{name} must be canonical modulo {modulus}")


def star_transport(r36: int, t584: int) -> tuple[int, int]:
    """Compute (X mod 72, X mod 73) from compatible bounded readings."""
    require_reading("r36", r36, 36)
    require_reading("t584", t584, 584)
    if (t584 - r36) % 4:
        raise ValueError("readings are incompatible on their mod-4 overlap")
    parity = ((t584 - r36) // 4) % 2
    return r36 + 36 * parity, t584 % 73


def restore_star_source(u72: int, v73: int, retained37: int) -> tuple[int, int, int]:
    """Restore (mod 36, mod 37, mod 584) from the complete target frame."""
    require_reading("u72", u72, 72)
    require_reading("v73", v73, 73)
    require_reading("retained37", retained37, 37)
    representative = u72 + 72 * ((u72 - v73) % 73)
    return u72 % 36, retained37, representative % 584


def calendar_transport(r260: int, v584: int, w23: int) -> int:
    """Compute X mod 11,960 directly from three compatible readings."""
    require_reading("r260", r260, 260)
    require_reading("v584", v584, 584)
    require_reading("w23", w23, 23)
    if (v584 - r260) % 4:
        raise ValueError("readings are incompatible on their mod-4 overlap")
    parity = ((v584 - r260) // 4) % 2
    u520 = r260 + 260 * parity
    quotient_class = (5 * (w23 - u520)) % 23
    return u520 + 520 * quotient_class


def restore_calendar_source(y11960: int, retained73: int) -> tuple[int, int, int]:
    """Restore (mod 260, mod 584, mod 23) from the complete target frame."""
    require_reading("y11960", y11960, 11_960)
    require_reading("retained73", retained73, 73)
    v584 = retained73 + 73 * ((y11960 - retained73) % 8)
    return y11960 % 260, v584, y11960 % 23


def factorization(n: int) -> dict[int, int]:
    if n < 1:
        raise ValueError("factorization expects a positive integer")
    factors: dict[int, int] = {}
    candidate = 2
    while candidate * candidate <= n:
        while n % candidate == 0:
            factors[candidate] = factors.get(candidate, 0) + 1
            n //= candidate
        candidate = 3 if candidate == 2 else candidate + 2
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def least_raw_added_modulus(source_period: int, target_period: int) -> int:
    """Least a such that target_period divides lcm(source_period, a)."""
    source_factors = factorization(source_period)
    answer = 1
    for prime, target_power in factorization(target_period).items():
        if source_factors.get(prime, 0) < target_power:
            answer *= prime**target_power
    assert math.lcm(source_period, answer) % target_period == 0
    return answer


def ambiguity_count(source_period: int, target_period: int) -> int:
    return target_period // math.gcd(source_period, target_period)


def star_number(index: int) -> int:
    return 6 * index * (index - 1) + 1


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    divisor = 3
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 2
    return True


def verify() -> dict[str, object]:
    star_period = math.lcm(36, 37, 584)
    assert star_period == 194_472
    star_input_pairs: set[tuple[int, int]] = set()
    for x in range(star_period):
        readings = (x % 36, x % 37, x % 584)
        u72, v73 = star_transport(readings[0], readings[2])
        assert (u72, v73) == (x % 72, x % 73)
        assert (u72 - v73) % 73 == (x // 72) % 73
        assert restore_star_source(u72, v73, readings[1]) == readings
        star_input_pairs.add((readings[0], readings[2]))
    assert len(star_input_pairs) == 5_256

    calendar_period = math.lcm(260, 584, 23)
    assert calendar_period == 873_080
    calendar_target_phases: set[int] = set()
    for x in range(calendar_period):
        readings = (x % 260, x % 584, x % 23)
        y11960 = calendar_transport(*readings)
        assert y11960 == x % 11_960
        assert restore_calendar_source(y11960, x % 73) == readings
        calendar_target_phases.add(y11960)
    assert len(calendar_target_phases) == 11_960

    # Explicit refusal checks: canonical ranges and overlap provenance.
    refusal_checks = 0
    for call, args in (
        (star_transport, (-1, 0)),
        (star_transport, (0, 584)),
        (star_transport, (0, 1)),
        (calendar_transport, (260, 0, 0)),
        (calendar_transport, (0, 584, 0)),
        (calendar_transport, (0, 0, 23)),
        (calendar_transport, (0, 1, 0)),
    ):
        try:
            call(*args)
        except (TypeError, ValueError):
            refusal_checks += 1
        else:
            raise AssertionError(f"malformed input was accepted: {call.__name__}{args}")

    calendar_ladder = []
    combined = 1
    for name, modulus in (
        ("Tzolkin", 260),
        ("Haab", 365),
        ("Venus model", 584),
        ("Mars model", 780),
        ("819-day", 819),
    ):
        prior = combined
        combined = math.lcm(combined, modulus)
        calendar_ladder.append(
            {
                "name": name,
                "modulus": modulus,
                "combined_period": combined,
                "new_resolution_factor": combined // prior,
            }
        )

    target_cases = []
    for source_period in (30_030, 18_980, 37_960):
        target_cases.append(
            {
                "source_period": source_period,
                "target_period": 11_960,
                "ambiguity": ambiguity_count(source_period, 11_960),
                "least_additional_raw_modulus": least_raw_added_modulus(
                    source_period, 11_960
                ),
            }
        )
    assert [case["ambiguity"] for case in target_cases] == [92, 46, 23]
    assert [case["least_additional_raw_modulus"] for case in target_cases] == [184, 184, 23]

    star_successors = []
    expected_anchors = [
        333,
        584,
        605,
        1_629,
        1_771,
        5_392,
        11_691,
        2_705,
        7_271,
        57_096,
        937,
        7_651,
        4_365,
        46_112,
    ]
    for index, expected in zip(range(2, 16), expected_anchors, strict=True):
        current = star_number(index)
        following = star_number(index + 1)
        source = (current - 1) * current
        target = (following - 1) * following
        anchor = least_raw_added_modulus(source, target)
        assert anchor == expected
        star_successors.append(
            {
                "index": index,
                "current_star": current,
                "next_star": following,
                "source_period": source,
                "target_period": target,
                "least_additional_raw_modulus": anchor,
                "factorization": {str(p): e for p, e in factorization(anchor).items()},
            }
        )

    hydra_period = math.lcm(260, 365, 18_980, 144_000)
    assert hydra_period == 136_656_000
    assert 2**24 < hydra_period
    assert math.lcm(260, 365, 584) == 37_960
    assert math.gcd(37_960, 11_960) == 520
    assert math.lcm(37_960, 23) == calendar_period
    assert math.lcm(11_960, 73) == calendar_period
    assert 1_872_000 == 98 * 18_980 + 11_960
    assert is_prime(11_959)

    return {
        "status": "PASS",
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "scope": (
            "Exhaustive finite arithmetic/model checks. No manuscript opcode, "
            "physical law, cryptographic security, or new Lean declaration is inferred."
        ),
        "source_pins": {
            "dccms_baseline": "40a76158970c41440ec7e4c10437fa1c2e271d03",
            "astrology_remastered": "ad0c8380f02b1d0c4f05a3e00726be3bc9680058",
            "lean_campaign": "dd9c6cfb341ad3747f148c336a015e2b80664e87",
            "nine65_session": "25bb6a7c464d97a439544d1d1e2d32f0baf3f6cb",
        },
        "counts": {
            "star_complete_states": star_period,
            "star_frame_roundtrips": star_period,
            "star_distinct_constructor_inputs": len(star_input_pairs),
            "calendar_complete_states": calendar_period,
            "calendar_frame_roundtrips": calendar_period,
            "calendar_distinct_target_phases": len(calendar_target_phases),
            "malformed_input_refusals": refusal_checks,
        },
        "constructors": {
            "star": {
                "input_moduli": [36, 584],
                "retained_modulus": 37,
                "output_moduli": [72, 73],
                "least_additional_raw_modulus": least_raw_added_modulus(1_332, 5_256),
                "joint_period": star_period,
            },
            "calendar": {
                "input_moduli": [260, 584, 23],
                "retained_modulus": 73,
                "output_modulus": 11_960,
                "joint_period": calendar_period,
            },
        },
        "target_resolution_cases": target_cases,
        "calendar_period_ladder": calendar_ladder,
        "star_successor_least_anchors": star_successors,
        "identities": {
            "calendar_260_365_584_lcm": 37_960,
            "calendar_target_gcd": 520,
            "complete_calendar_frame_period": calendar_period,
            "long_count_rounds": 98,
            "long_count_remainder": 11_960,
            "hydra_full_phase_period": hydra_period,
            "packed_24_bit_capacity": 2**24,
            "surviving_table_11959_is_prime": True,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, help="write the JSON record here")
    args = parser.parse_args()
    report = verify()
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(encoded, encoding="utf-8")
    print(encoded, end="")


if __name__ == "__main__":
    main()
