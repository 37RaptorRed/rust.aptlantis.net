# Fedora APTlantis Server Context

Baseline captured 2026-07-10.

## Host

| Field | Value |
|---|---|
| Server | Dell home server |
| SSH | `ssh herb@192.168.0.80` |
| Storage noted | 1TB HDD |
| Python | 3.14.6 |
| Node | v24.18.0 |
| Rustup | 1.29.0 |
| Docker | 29.6.1 |

## Server directories

| Purpose | Path |
|---|---|
| Websites family/root | `/home/herb/aptlantis` |
| aptlantis.net site | `/home/herb/aptlantis.net` |
| rust.aptlantis site/data | `/home/herb/rust.aptlantis` |
| aptlantis.studio site | `/home/herb/aptlantis.studio` |

## Windows-local mirror resources

The authoritative local Rust mirror source currently lives on Windows at `A:\rust-lang`.

| Resource | Path | Notes |
|---|---|---|
| Full crates.io mirror | `A:\rust-lang\crates` | Full crates.io mirror with sidecars |
| crates.io index | `A:\rust-lang\crates.io-index` | Cloned crates.io index |
| Rustup mirror root | `A:\rust-lang\rustup` | Synced from `rsync://mirror.nyist.edu.cn/rustup/` |
| Rustup dist | `A:\rust-lang\rustup\dist` | Distribution payloads |
| Rustup orig | `A:\rust-lang\rustup\orig` | Original payloads |
| Rustup nightly | `A:\rust-lang\rustup\nightly` | Nightly payloads |

## Docker responsibilities

Docker on Fedora is expected to run:

- Caddy webserver.
- `cloudflared` tunnel process.
- Cron-like or scheduled jobs.
- Rust servers or job images built for this project.

The first repo scaffold for this is under `deploy/fedora/`.

## Cloudflare

| Field | Value |
|---|---|
| Tunnel name | `aptlantis-fedora` |
| Tunnel ID | `693422ea-bfa2-44fd-b7ca-ac96e5aed100` |

Published application routes:

| Hostname | Origin service |
|---|---|
| `aptlantis.net` | `http://localhost:8080` |
| `www.aptlantis.net` | `http://localhost:8080` |
| `aptlantis.studio` | `http://localhost:8181` |
| `www.aptlantis.studio` | `http://localhost:8181` |

`rust.aptlantis.net` is reserved in the local Caddy scaffold at `http://localhost:8282`, but the Cloudflare route is intentionally commented until the public route is ready.

## Initial deployment scaffold

`deploy/fedora/compose.yaml` defines:

- `caddy`, bound only to localhost ports `8080`, `8181`, and reserved `8282`.
- `cloudflared`, using `CLOUDFLARE_TUNNEL_TOKEN` from `.env`.
- `rust-pipeline`, behind the `jobs` Compose profile so it can be run intentionally.

Example commands from the repo root on Fedora:

```bash
cd /home/herb/rust.aptlantis
cp deploy/fedora/.env.example deploy/fedora/.env
# edit deploy/fedora/.env with the real tunnel token
docker compose --env-file deploy/fedora/.env -f deploy/fedora/compose.yaml up -d caddy cloudflared
docker compose -f deploy/fedora/compose.yaml --profile jobs run --rm rust-pipeline
```

The pipeline container expects:

- Project data at `/home/herb/rust.aptlantis`, mounted as `/srv/rust.aptlantis`.
- Rust mirror input at `/mnt/rust-lang`, mounted read-only from the Fedora host.

The exact Windows-to-Fedora sync path for `A:\rust-lang` is not yet defined. Until that is decided, `/mnt/rust-lang` is a placeholder mount target for the transferred mirror input.
