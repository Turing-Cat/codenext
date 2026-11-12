//! Regression coverage for overrides split across CLI command boundaries.

use super::try_parse_from;
use crate::Subcommand;
use crate::mcp_cmd::McpSubcommand;
use pretty_assertions::assert_eq;

#[test]
fn preserves_scoped_overrides_and_forwarded_arguments() {
    let cli = try_parse_from([
        "codex",
        "--config=a=1",
        "mcp",
        "-cb=2",
        "add",
        "example",
        "-c",
        "a=3",
        "--",
        "program",
        "-c",
        "payload",
    ])
    .expect("parse");
    assert_eq!(cli.config_overrides.raw_overrides, ["a=1", "b=2", "a=3"]);
    let Some(Subcommand::Mcp(mcp)) = cli.subcommand else {
        panic!("expected mcp");
    };
    assert_eq!(mcp.config_overrides.raw_overrides, ["a=3"]);
    let McpSubcommand::Add(add) = mcp.subcommand else {
        panic!("expected add");
    };
    assert_eq!(
        add.transport_args.stdio.expect("stdio transport").command,
        ["program", "-c", "payload"],
    );
}

#[test]
fn codenext_invocation_shows_codenext_in_usage() {
    let error = try_parse_from(["codenext", "--help"]).expect_err("help should short-circuit");
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
    let help = error.to_string();
    assert!(
        help.contains("Usage: codenext [OPTIONS] [PROMPT]"),
        "{help}"
    );
    assert!(
        help.contains("codenext [OPTIONS] <COMMAND> [ARGS]"),
        "{help}"
    );
}

#[test]
fn codex_alias_keeps_codex_in_usage() {
    let error = try_parse_from(["codex", "--help"]).expect_err("help should short-circuit");
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
    let help = error.to_string();
    assert!(help.contains("Usage: codex [OPTIONS] [PROMPT]"), "{help}");
}
