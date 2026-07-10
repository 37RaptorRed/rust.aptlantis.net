# rust.aptlantis.net Mockup Notes

Status: concept mockups  
Date: 2026-07-10  
Input references: `PROJECT.md`, `README.md`, `D:\INDEX.md`, `D:\Development.manifest.toml`, two `.crate.json` sidecars, and Aptlantis Studio screenshots.

## Product Frame

`rust.aptlantis.net` should feel like a public Rust ecosystem observatory built from a serious local mirror, not just a file browser.

The raw mirror is the source material. The user-facing product is the explorable record of crates.io over time:

- Current mirror health and freshness.
- Immutable snapshots with provenance.
- Crate and version metadata browsing.
- Yanked, dependency, license, maintainer, and publish trend analytics.
- Torrent and research-dataset distribution.
- Evidence that CloneCratesio, ArchiveHasher, AAMHS, and the Rust pipeline are each doing their job.

## Relationship To Aptlantis Sites

| Site | Role |
| --- | --- |
| `aptlantis.net` | Hub for Aptlantis domains, mirrors, project groups, and durable published surfaces. |
| `rust.aptlantis.net` | Public Rust mirror, snapshot archive, analytics surface, and research-dataset release index. |
| `aptlantis.studio` | Teaching, exploration, project demos, standards, and hands-on testing. |

The visual language should inherit from Aptlantis Studio: dark operational canvas, cyan status signal, magenta accent, squared cards, compact controls, and evidence-first page structure.

## Mockup Set

The companion storyboard is:

`docs/mockups/rust-aptlantis-storyboard.html`

It contains four screens:

1. **Observatory Home**  
   First screen for the whole site. Shows the current snapshot, freshness, validation, mirror scale, latest run, and quick paths into crates, snapshots, analytics, torrents, and research releases.

2. **Crate Detail**  
   Uses the sidecar shape as the UI unit. Shows a crate/version header, checksum, yanked state, Rust version, publish time, index path, dependencies, and source download. This is where a visitor can inspect one version without reading raw JSON.

3. **Run Health**  
   Turns the build report into an operator and public trust surface: stage timeline, statuses, validation notes, counts, duration, and downstream publish/hash/package state.

4. **Research Dataset Release**  
   Shows the periodic release product: one citable snapshot bundle with metadata, dependency graph, yanked history, hashes, signatures, analytics, changelog, torrent, magnet, and reproducibility notes.

## Data Surfaces To Prioritize

### Near-term fixture fields

From the sidecars:

- `name`
- `vers`
- `cksum`
- `crate_file`
- `crate_url`
- `deps[]`
- `features`
- `index_path`
- `pubtime`
- `rust_version`
- `yanked`

From build reports:

- run id
- started / finished / duration
- new crates
- updated crates
- yanked versions
- validation status
- snapshot status
- hash/signature status
- website status
- torrent status
- health state
- freshness age

From snapshot manifests:

- snapshot id
- timestamp
- record counts
- hash manifest path
- torrent path
- research release membership
- previous snapshot id
- current pointer state

### Later analytics islands

- Growth over time.
- Yanked version share over time.
- New vs yanked releases by snapshot.
- Dependency graph growth.
- Most active maintainers.
- License distribution.
- SemVer adoption.
- Dormant crates and revivals.
- MSRV trends once source data supports it.

## Navigation Model

Top nav should stay compact:

- Overview
- Crates
- Snapshots
- Analytics
- Health
- Datasets

Secondary local tabs should handle page-level detail. Avoid making the top nav carry every project idea.

## Visual Direction

- Dark base with black and near-slate surfaces.
- Cyan for live/current/valid.
- Green for verified.
- Amber for degraded, freshness warning, or partial.
- Magenta for research, provenance, and special highlighted insight.
- Avoid a one-color cyan-only interface; rust.aptlantis.net should feel adjacent to Aptlantis Studio but more data-dense.
- Cards should be compact, squared, and operational. Use cards for repeated items and tools only.
- The first viewport should be useful immediately: mirror state, scale, and where to go next.

## Implementation Notes For Later

- Astro can statically generate crate/version pages from precomputed JSON.
- React islands should hydrate only interactive filters, charts, dependency graph exploration, and command/dataset builders.
- Tailwind should encode design tokens instead of one-off colors.
- The site should not recompute large registry analytics in the browser. It should consume precomputed compact JSON from the pipeline.
- The mockup uses static CSS only. It is not an Astro implementation.

