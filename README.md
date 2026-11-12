<p align="center"><strong>Codenext</strong> — a local coding agent for your terminal, forked from <a href="https://github.com/openai/codex">OpenAI Codex CLI</a>.</p>

---

## About this fork

Codenext is a personal fork of [OpenAI Codex CLI](https://github.com/openai/codex).
It is **not** affiliated with or endorsed by OpenAI. The upstream code remains
licensed under [Apache-2.0](LICENSE); see [NOTICE](NOTICE) for attribution.

The fork tracks upstream and keeps local changes small. To see what it adds on
top of upstream:

```shell
git fetch upstream
git log --oneline upstream/main..main
```

## Build and run

Codenext builds from source with Cargo. Requirements are macOS 12+,
Ubuntu 20.04+/Debian 10+, or Windows 11 via WSL2, with at least 4 GB of RAM.
See [docs/install.md](docs/install.md) for the full setup.

```shell
git clone https://github.com/Turing-Cat/codenext.git
cd codenext/codex-rs
cargo build
cargo run --bin codenext -- "explain this codebase to me"
```

`cargo build` produces two binaries from the same entry point: `codenext`
(primary) and `codex` (compatibility alias).

To run the test suite:

```shell
just test
```

## Authentication

Codenext uses the same login and configuration as upstream Codex CLI:

- Run the CLI and choose **Sign in with ChatGPT**, or
- Provide an API key via `CODEX_ACCESS_TOKEN` / `OPENAI_API_KEY`.

Configuration lives in `$CODEX_HOME` (default `~/.codex`), and the interactive
UI is started by running `codenext` (or the `codex` alias).

## Naming status

The executable is now `codenext`, with `codex` kept as an alias for existing
scripts. Cargo crates and configuration identifiers keep their upstream names
so that upstream changes stay easy to merge:

| Item | Name |
| --- | --- |
| Project / repository | Codenext |
| Executable | `codenext` (primary), `codex` (alias) |
| Cargo crates | `codex-*` (unchanged) |
| Config directory | `~/.codex` / `CODEX_HOME` (unchanged) |
| Environment variables | `CODEX_*` (unchanged) |

Renaming the crates or configuration directory is a separate, larger change.

## Docs

- [Contributing](./docs/contributing.md)
- [Installing & building](./docs/install.md)
- [Configuration](./docs/config.md)
- [Upstream Codex documentation](https://developers.openai.com/codex)

## License

This repository is licensed under the [Apache-2.0 License](LICENSE).
