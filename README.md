<h1 align="center">Auttacut (อัตคัด)</h1>

<p align="center">
  <b>อัตคัด ตัวตัดสำหรับคนไม่มีเงินซื้อหรือเช่าโปรแกรม</b><br>
  A free video editor for people who can't afford to buy or rent one.
</p>

<p align="center">
  <a href="#license-and-credits"><img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-8b5cf6"></a>
  <img alt="Written in pure Rust" src="https://img.shields.io/badge/pure-Rust-6a3fd6?logo=rust&logoColor=white">
  <img alt="Runs on macOS, Windows and Linux" src="https://img.shields.io/badge/runs%20on-macOS%20%7C%20Windows%20%7C%20Linux-8b5cf6">
</p>

Auttacut is based on [FilmCraft](https://github.com/storytold/filmcraft) by the ArtCraft team: a
clean-room, pure-Rust non-linear editor with a multi-track timeline, color, effects, audio
mixing, captions and Premiere/Final Cut XML interchange. Auttacut tunes it for fast, daily
short-form news editing, with Thai text first.

It is not an ArtCraft product and is not endorsed by the ArtCraft team.

## What Auttacut adds

- **Thai line breaking.** Thai titles and captions wrap between words instead of running out of
  the box.

More is on the way: one-click channel templates that lay out a whole piece (title card, brand
frame, voice-over, ducked source audio, music, end card) and export it with the right name.

## Get started

```sh
cargo run --release -p filmcraft            # the desktop app
cargo run --release -p filmcraft-cli -- help # drive it from the shell or an agent
```

A Final Cut Pro 7 XML (for example one exported from Premiere) opens with
`filmcraft-cli import timeline.xml`. For everything FilmCraft can do, see its
[README](https://github.com/storytold/filmcraft#readme) and [docs/](docs/).

Report problems on [GitHub issues](https://github.com/organeizer/Auttacut/issues).

## License and credits

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at
your option. Copyright (c) 2026 ArtCraft Team and the FilmCraft contributors, and the Auttacut
contributors. Required notices are in [NOTICE](NOTICE); every bundled font, icon and asset is
listed with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

The ArtCraft name and logos are trademarks of the ArtCraft Team. As their brand licence requires
of modified versions, Auttacut ships none of them; "FilmCraft" and "ArtCraft" appear only to say
where Auttacut comes from.

<sub>Adobe and Premiere Pro are trademarks or registered trademarks of Adobe Inc. Auttacut is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Adobe Inc.</sub>
