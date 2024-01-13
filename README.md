# Tauri Chess Engine

![License](https://img.shields.io/github/license/zS1L3NT/tauri-chess?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/tauri-chess?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/tauri-chess?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/tauri-chess?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/tauri-chess?style=for-the-badge)

This is a chess engine written in Tauri, where the frontend (the board and pieces) is in React and the backend (move generation) is written in Rust. This is NOT a chess AI, meaning that all this repository contains is code to determine the next possible legal moves for a specific chess position.

In the future, I plan to improve the performance of this engine with bitboards and bitwise operations instead of using Vectors. I also plan to train a Supervised Learning model on games played by players above the LiChess rating of 1600 to be able the evaluate the score of chess positions, then use alpha-beta pruning to find the best move for a specific position. I could possibly add an evaluation function too to improve the performance of the engine. For now, these features will be put off so I can focus on other more important projects. 

## Motivation

I seriously love the game of Chess. This is my third repository where I attempt to rebuild a chess engine in yet another language. From [Java](https://github.com/zS1L3NT/java-chess), to [Typescript](https://github.com/zS1L3NT/web-angular-chess) and now to Rust. I was also heavily inspired by how Sebastian Lague implemented his chess engine in [this](https://www.youtube.com/watch?v=U4ogK0MIzqk) video.

## Features

-	React UI with drag & drop of pieces
-	Engine to determine all legal moves in a specific position

## Credits

I did a LOT of referring to [this](https://www.chessprogramming.org/Perft_Results) website to get the perft results for the move generation testing. This is to test that my chess engine was producing the correct list of legal moves for any possible board configuration.
I also did something similar but not exactly the same as how Sebastian Lague cached attack lines in [this](https://www.youtube.com/watch?v=U4ogK0MIzqk) video. This video was my main inspiration for this project.

## Tests

All tests, including perft and struct tests can be run with the command. The perft tests may take a long time to run (the longest perft tests takes about 7 minutes to run on my computer).

```bash
$ cargo test
```

## Built with

-   Rust
    -   Tauri
        -   [![tauri](https://img.shields.io/badge/tauri-1.0.0-blue?style=flat-square)](https://crates.io/crates/tauri/1.0.0)
    -   Miscellaneous
        -   [![indexmap](https://img.shields.io/badge/indexmap-1.9.1-blue?style=flat-square)](https://crates.io/crates/indexmap/1.9.1)
        -   [![serde](https://img.shields.io/badge/serde-1.0-blue?style=flat-square)](https://crates.io/crates/serde/1.0)
        -   [![serde_json](https://img.shields.io/badge/serde_json-1.0-blue?style=flat-square)](https://crates.io/crates/serde_json/1.0)
-   Typescript
    -   Typescript
        -   [![@types/node](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/@types/node?style=flat-square)](https://npmjs.com/package/@types/node)
        -   [![@types/react](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/@types/react?style=flat-square)](https://npmjs.com/package/@types/react)
        -   [![@types/react-dom](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/@types/react-dom?style=flat-square)](https://npmjs.com/package/@types/react-dom)
        -   [![typescript](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/typescript?style=flat-square)](https://npmjs.com/package/typescript)
    -   React
        -   [![react](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/react?style=flat-square)](https://npmjs.com/package/react)
        -   [![react-dom](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/react-dom?style=flat-square)](https://npmjs.com/package/react-dom)
    -   Redux
        -   [![@reduxjs/toolkit](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/@reduxjs/toolkit?style=flat-square)](https://npmjs.com/package/@reduxjs/toolkit)
        -   [![react-redux](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/react-redux?style=flat-square)](https://npmjs.com/package/react-redux)
    -   Tauri
        -   [![@tauri-apps/api](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/@tauri-apps/api?style=flat-square)](https://npmjs.com/package/@tauri-apps/api)
        -   [![@tauri-apps/cli](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/@tauri-apps/cli?style=flat-square)](https://npmjs.com/package/@tauri-apps/cli)
    -   Vite
        -   [![@vitejs/plugin-react](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/@vitejs/plugin-react?style=flat-square)](https://npmjs.com/package/@vitejs/plugin-react)
        -   [![vite](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/dev/vite?style=flat-square)](https://npmjs.com/package/vite)
    -   Miscellaneous
        -   [![framer-motion](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/framer-motion?style=flat-square)](https://npmjs.com/package/framer-motion)
        -   [![use-async-effect](https://img.shields.io/github/package-json/dependency-version/zS1L3NT/tauri-chess/use-async-effect?style=flat-square)](https://npmjs.com/package/use-async-effect)
