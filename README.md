# Rustle
![wakatime](https://wakatime.com/badge/user/018e2f99-047a-455f-8d81-d71f9269c7ce/project/29d1aebe-1b1b-4806-aca3-e0da16a2087d.svg?style=for-the-badge)

[![forthebadge](data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMTAuMjQ5OTk2MTg1MzAyNzMiIGhlaWdodD0iMzUiIHZpZXdCb3g9IjAgMCAyMTAuMjQ5OTk2MTg1MzAyNzMgMzUiPjxyZWN0IHdpZHRoPSIxMzkuNDE2NjY0MTIzNTM1MTYiIGhlaWdodD0iMzUiIGZpbGw9IiNGRjhDMDAiLz48cmVjdCB4PSIxMzkuNDE2NjY0MTIzNTM1MTYiIHdpZHRoPSI3MC44MzMzMzIwNjE3Njc1OCIgaGVpZ2h0PSIzNSIgZmlsbD0iI0ZGNjM0NyIvPjx0ZXh0IHg9IjY5LjcwODMzMjA2MTc2NzU4IiB5PSIyMS41IiBmb250LXNpemU9IjEyIiBmb250LWZhbWlseT0iJ1JvYm90bycsIHNhbnMtc2VyaWYiIGZpbGw9IiNGRkZGRkYiIHRleHQtYW5jaG9yPSJtaWRkbGUiIGxldHRlci1zcGFjaW5nPSIyIj5QUk9DUkFTVElOQVRFPC90ZXh0Pjx0ZXh0IHg9IjE3NC44MzMzMzAxNTQ0MTg5NSIgeT0iMjEuNSIgZm9udC1zaXplPSIxMiIgZm9udC1mYW1pbHk9IidNb250c2VycmF0Jywgc2Fucy1zZXJpZiIgZmlsbD0iI0ZGRkZGRiIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC13ZWlnaHQ9IjkwMCIgbGV0dGVyLXNwYWNpbmc9IjIiPlBBTklDPC90ZXh0Pjwvc3ZnPg==)](https://forthebadge.com)

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
- [ ] Parallel / distributed crawling
- [ ] Obey `robots.txt` & Use Google Webcrawler User Agent
