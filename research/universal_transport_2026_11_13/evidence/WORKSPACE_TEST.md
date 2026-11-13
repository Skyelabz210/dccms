# Rust workspace verification

The host environment did not expose `cargo`, so the unchanged DCCMS checkout
was mounted into the official Rust 1.89 container. The host CA bundle was
mounted read-only so Cargo could verify the environment's TLS proxy chain.

Command:

```bash
docker run --rm \
  -v "$PWD:/work" \
  -v /etc/ssl/certs/ca-certificates.crt:/etc/ssl/certs/ca-certificates.crt:ro \
  -w /work rust:1.89 \
  cargo test --workspace --locked
```

Result: **PASS**

| Target | Passed | Failed |
|---|---:|---:|
| `dccms_atlas` unit tests | 553 | 0 |
| `dccms_atlas/tests/h4_real_pixel_discharge.rs` under default features | 0 | 0 |
| `dresden_codex` unit tests | 94 | 0 |
| `prime_hunt` unit tests | 22 | 0 |
| Doc tests | 0 | 0 |
| **Total** | **669** | **0** |

The optional SLUB feature, proof assistants, and external repositories are not
part of this command. Their earlier scoped results are pinned in
`SOURCE_MANIFEST.md` and the original NINE65 session packets.
