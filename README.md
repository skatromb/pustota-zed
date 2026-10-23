# pustota for Zed

[pustota](https://github.com/pustota-theme/pustota) theme by Nikita Sobolev, ported to Zed.

Install: open `zed: extensions` and search for `pustota`.

## Update colors

```sh
git -C pustota checkout 0.32.0  # upstream tag
cargo run
```

## Release

1. Bump `version` in `extension/extension.toml` and commit.
2. `git tag v0.2.0 && git push origin v0.2.0`

`.github/workflows/release.yml` opens a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions). It needs a `COMMITTER_TOKEN` secret, a classic token with `repo` and `workflow` scopes.
