# Rustle

![wakatime](https://wakatime.com/badge/user/018e2f99-047a-455f-8d81-d71f9269c7ce/project/29d1aebe-1b1b-4806-aca3-e0da16a2087d.svg?style=for-the-badge)
![Static Badge](https://img.shields.io/badge/%E2%99%A5-orange?style=for-the-badge&label=Built%20With&labelColor=darkorange)

## Usage

First, ensure that the config file exists at the following location (per your OS):

| OS    | Path                                                   |
| ----- | ------------------------------------------------------ |
| Linux | `$HOME/.config/Rustle/config.toml`                     |
| MacOS | `$HOME/Library/Application Support/Rustle/config.toml` |

Then just use:

`rustle`

### Configuration

Example `config.toml` file:

```toml
origin_url = "https://example.com"
depth = 6
database_name = "crawler"
```

### Logging

- To configure logging, this program uses the `RUST_LOG` environment variable, with options:

  - `error`
  - `warn`
  - `info`
  - `debug`
  - `trace`

- **Example:**

  ```bash
  RUST_LOG=info rustle
  ```

## Roadmap

- [x] Abstract code & functionality into structs & other files
- [x] Use SQLite to store information about websites, instead of downloading HTML
- [x] Recursion fix, specify depth
- [x] config file parsing to specify origin url & depth
- [x] Parallel / distributed crawling
- [x] Obey `robots.txt`
