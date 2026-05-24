# MeetCal CLI

Rust command line tool for querying MeetCal lifting data from Convex.

## Install

### Homebrew

```sh
brew tap meetcal/tap
brew install meetcal
```

Upgrade later:

```sh
brew update
brew upgrade meetcal
```

### Cargo

Install the latest release from GitHub:

```sh
cargo install --git https://github.com/meetcal/meetcal-cli.git meetcal
```

Install from a local checkout:

```sh
git clone https://github.com/meetcal/meetcal-cli.git
cd meetcal-cli
cargo install --path .
```

Requirements: [Rust](https://www.rust-lang.org/tools/install) 1.85+.

## Verify

```sh
meetcal --help
meetcal --version
```

## Commands

### `search`

Search for an athlete by name. Prints meet results, comp PRs, and make rates.

```sh
meetcal search "Maddisen Mohnsen"
```

### `meet`

Search meet entries by meet name. Optionally filter by session number and platform.

```sh
meetcal meet "2026 VIRUS Weightlifting Series 1"
meetcal meet "2026 VIRUS Weightlifting Series 1" --session-number 1 --session-platform red
```

Options:

- `--session-number`, `-s`: Session number
- `--session-platform`, `-p`: Platform (`red`, `white`, `blue`, `stars`, `stripes`, `rogue`)

### `records`

Search records by age group, gender, and federation.

```sh
meetcal records --age Senior --gender Men --federation USAW
```

Options:

- `--age`, `-a`: Age group
- `--gender`, `-g`: Gender
- `--federation`, `-f`: `IWF`, `USAW`, `USAMW`, or `UMWF`

### `standards`

Search USAW A/B standards for an age group and gender.

```sh
meetcal standards --age Senior --gender Men
```

Options:

- `--age`, `-a`: Age group
- `--gender`, `-g`: Gender

### `qualifying-totals`

Search qualifying totals for an age group, gender, and event.

```sh
meetcal qualifying-totals --age Senior --gender Men --event Nationals
```

Options:

- `--age`, `-a`: Age group
- `--gender`, `-g`: Gender
- `--event`, `-e`: Event name

### `nat-rankings`

Search national rankings for a weight class and federation.

```sh
meetcal nat-rankings "Junior Women's 77kg" --federation USAW
```

Options:

- `--federation`, `-f`: `IWF`, `USAW`, `USAMW`, or `UMWF`

### `intl-rankings`

Search international rankings for an age group, gender, and meet.

```sh
meetcal intl-rankings --age Senior --gender Men --meet Worlds
```

Options:

- `--age`, `-a`: Age group
- `--gender`, `-g`: Gender
- `--meet`, `-m`: Meet name

### `wso-records`

Search WSO records by age group, gender, and WSO region.

```sh
meetcal wso-records --age Senior --gender Men --wso Carolinas
```

Options:

- `--age`, `-a`: Age group (e.g. `U17`, `Junior`, `Senior`, `Masters 35`)
- `--gender`, `-g`: `Men` or `Women`
- `--wso`, `-w`: WSO region (e.g. `Carolina`, `Florida`)

## Development

```sh
cargo test
cargo run -- search "Maddisen Mohnsen"
cargo build --release
```

Release builds and Homebrew publishing steps are documented in [BREW.md](BREW.md).
