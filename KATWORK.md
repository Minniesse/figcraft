# KatWork fork of FigCraft

This repository is the KatWork fork of FigCraft (https://github.com/storytold/figcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on FigCraft by the ArtCraft team (https://github.com/storytold/figcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `ef2a9eeefc7af519fe7c26cc4c4b31d2284b09af`
- Upstream `storytold/figcraft` head at that time: `ef2a9eeefc7af519fe7c26cc4c4b31d2284b09af`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): font-download feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): collab feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): plugins feature, on by default` | draft, not opened (in the fork PR body) |
| `feat(katwork): hide the UI of compiled-out features` | draft, not opened (part of the collab and plugins drafts) |

## The KatWork build

Three Cargo features, each on by default so an upstream build is unchanged, are off in the KatWork build:

- `collab` (figcraft-engine, figcraft-mcp, figcraft-web; forwarded by figcraft-ui-egui and the CLI): live sessions and
  online files. With it off every `collab.*` command is unknown, and figcraft-collab-net (the WebSocket and TLS client,
  tungstenite, rustls, webpki-roots) is not built; figcraft-collab, the protocol core with no networking, stays.
- `plugins` (figcraft-engine; forwarded by figcraft-ui-egui, figcraft-mcp, the CLI and the web crate): the WebAssembly
  plugin sandbox (figcraft-plugin, wasmi). With it off every `plugin.*`, `plugins.*`, `plugin.directory.*` and
  `widget.*` command is unknown, the Plugins menu and windows are gone, a widget saved in a file is a plain frame, and
  `figcraft-cli plugin publish` does not exist.
- `font-download` (figcraft-engine; forwarded likewise): the font store's network client (craft-font-store, ureq).
  With it off `font.download` and `font.downloads` are unknown and no font is ever downloaded.

With the features off the UI shows none of what they leave out (figcraft-ui-egui, so the web build): no Share
button or people in the right panel, no Online section, "Join with a code" field or sharing tip on the Home screen, no
sharing server or invite-link setting in Preferences, no Plugins menu, no Plugins & widgets tab in the Actions palette,
no plugin windows, panels or widget menus, and no font Download button. The menu, palette and shortcut entries of the
removed commands are gone with them (catalog and keymap). Tests: `crates/ui-egui/src/katwork_hidden_tests.rs` and
`missing_fonts_tests::without_font_download_no_family_offers_a_download`, run with every feature off.

The CLI: `cargo build --locked --release -p figcraft-cli --no-default-features`. The web build: trunk with
`data-cargo-no-default-features` on `apps/figcraft-web` (no extra features). Both builds also set
`FIGCRAFT_DEFAULT_SERVER` to empty in the build environment (`FIGCRAFT_DEFAULT_SERVER= cargo build ...`): the engine
reads it with `option_env!` at compile time, and an empty value leaves no default sharing server in the binary
(with `collab` off nothing would use it anyway). `scripts/katwork-profile.sh` (CI job `katwork-profile`) checks that
the gated crates are not in either build graph, and `crates/engine/tests/katwork_profile.rs` that the commands are
unknown.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/figcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. The Windows executable's version information named ArtCraft as its company; it now names the vendor
the installer and the Linux packages already name (Learning Machines LLC). No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.figcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The nightly channel's messages and tests that name ArtCraft Launcher (`build_flavor.rs`, `profile_root.rs`), the launcher's
  check in the Windows install-policy script, and the Flatpak permission for the launcher's version folder
  (`xdg-data/ArtCraft/versions`): they name a separate program and its folders, the upstream nightly builds' updater; KatWork
  builds no nightly.
- The upstream CI and release workflows (`release.yml` secret name, `windows-arm64.yml` install path): upstream CI
  configuration, unchanged; KatWork does not run them.
- The "Based on" line in the About window's code (`crates/ui-egui/src/dialogs.rs`): the line this fork must show.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/figcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-128.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-16.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-256.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-32.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-48.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-64.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-128.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-16.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-256.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-32.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-48.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly-64.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-nightly.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/figcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.figcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.figcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `crates/svg/tests/corpus/arcs.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/blend_visibility.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/clip_mask.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/colors.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/css.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/fill_rule.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/filters.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/gradients.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/icon.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/illustration.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/image.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/opacity.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/open_subpaths.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/patterns.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/primitives.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/strokes.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/text.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/transforms.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/units.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/use_symbol.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/viewbox_meet.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `crates/svg/tests/corpus/viewbox_slice.svg`: kept: test fixture of the SVG importer (viewed: shapes and text, no ArtCraft mark)
- `docs/images/art-app-home-dark.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-app-home.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-app-trail.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-dashboard.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-ds-colour.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-glass-weather.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/art-vector-marks.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-1-hero.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-2-components.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-3-prototype.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-4-devmode.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-5-vector.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-6-dashboard-light.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-7-collab.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-8-board.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/shot-9-devmode-measure.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
