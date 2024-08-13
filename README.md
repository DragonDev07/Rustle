# Rustle

## Usage

- `rustle`

### Logging

- To configure logging, this program uses the `RUST_LOG` enviroment variable, with options:

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

- [ ] Abstract code & functionality into structs & other files
- [ ] Use SQLite to store information about websites, instead of downloading HTML
- [ ] Recursion fix, specify depth
- [ ] `clap` implementation to specify origin url & depth
- [ ] Parallel / distributed crawling
- [ ] Obey `robots.txt` & Use Google Webcrawler User Agent
