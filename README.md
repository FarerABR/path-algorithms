# Path Algorithms

Path Algorithms is a Tauri desktop app for visualizing pathfinding on a 2D grid.
You can generate a random maze, set start/destination points, add walls, and solve
the grid using:

- Depth-First Search (DFS)
- Breadth-First Search (BFS)
- A* (A-star)

The app also shows visited/opened nodes and execution time for each run.

## Use the app from Releases

You can use the app directly from the compiled binaries published in
[GitHub Releases](https://github.com/FarerABR/path-algorithms/releases).
Download the package for your OS and run it without building locally.

## Run locally (development)

Prerequisites:

- Rust toolchain
- Tauri dependencies for your platform

Then run:

```bash
cargo tauri dev
```
