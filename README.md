# Rustle
![wakatime](https://wakatime.com/badge/user/018e2f99-047a-455f-8d81-d71f9269c7ce/project/29d1aebe-1b1b-4806-aca3-e0da16a2087d.svg?style=for-the-badge)

## Usage

- `rustle`

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
- [ ] Use SQLite to store information about websites, instead of downloading HTML
- [x] Recursion fix, specify depth
- [ ] `clap` implementation to specify origin url & depth
- [x] Parallel / distributed crawling
- [ ] Obey `robots.txt` & Use Google Webcrawler User Agent
