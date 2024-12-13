## About This Repo

I really love **uv** and **polars**, two fantastic Rust-based Python packages and I wanted to track their adoption. This app automates the process of fetching download statistics for Python packages from [pepy.tech](https://pepy.tech/) and displaying them on a [static GitHub Pages site](https://benkulcsar.github.io/).

### How It Works

1. A scheduled **GitHub Actions** workflow builds and runs the Rust app.
2. The Rust app fetches download statistics and pushes the data to a **GitHub Gist**.
3. A static **GitHub Pages** site reads the Gist and displays the latest statistics.

!["Flow"](img/rustybears_flow.png?v=746&s=325)