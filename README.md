# WoWP

## ![Rust](https://github.com/rp4rk/WoWP/workflows/Rust/badge.svg)

---

Combat Log Parser for World of Warcraft

WoWP is a combat log parser for World of Warcraft that aims to provide a fast conversion from the retail World of Warcraft combat log format into JSON. This is a huge barrier to entry for people wishing to build gameplay analysis tooling, so by providing a prebuilt way to do that into a sane format, we're hoping to see more innovation in the space.

## About

---

WoWP will fully parse any modern World of Warcraft log file, this is commonly known as the `WoWCombatlog.txt` file. The result is a stream of JSON data that will

## How to Build

- Install [Cargo/Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Clone this repository
- Run `cargo build --release`
- The binary can be found in `target/release`

## Usage Instructions

---

Running WoWP requires three different components,

- WoWP Binary, see [How to Build](#how-to-build) or the releases
- Event templates, in order to correctly match events and their composite data to a JSON structure, see the src folder for all the maintained templates.
- A `WoWCombatLog.txt` file

Once you have these components, simply execute the binary and pass in the relevant flags,

`./wowp --event-structures="./event_templates"`

After which, JSON will start being streamed to stdout for you to use, e.g.

`./wowp --event-structures="./event_templates" > combat-log.json`

## Flags

- `-p` will parse trash events, otherwise only encounters are parsed along with zone changes and the combat log version event.
  - Defaults to false
- `--event-structures="path"` The path to a set of complete event structures
  - Defaults to `./event_templates`
- `--combat-log="path"` The path to a combat log file
- Defaults to `./WoWCombatLog.txt`

## FAQ

---

- Why Rust?
  - Excellent parser combinator libraries such as [nom](https://github.com/Geal/nom) - these are incredibly helpful for parsing recursive structures like the WoWCombatLog format.
- Can this replace WarcraftLogs/WoWAnalyzer/`<Other service>` that I use?
  - No, WoWP could fit nicely in their processing pipelines though ;)
- Can I use this for `<My Service>`?
  - The project uses the AGPL license, so most likely yes.
