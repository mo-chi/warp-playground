# warp playground

Rust の WEB フレームワークの warp の検証用リポジトリ

## 構成情報

Rust: 1.78.0
warp: 0.3.7

## ファイル構成

```sh
.
├── README.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain
├── src/
│   ├── main.rs
│   ├── routes.rs
│   ├── controllers.rs
│   └── models.rs
└── target/
```

## ローカル環境構築

**パッケージのインストールする**

```sh
cargo install cargo-watch
```

**ライブリロード用のライブラリをインストールする**

```sh
cargo install --path .
```

**warp を起動する**

```sh
cargo watch -x run
```

**動作確認**

```sh
curl http://localhost:8000
```

```sh
curl http://localhost:8000/users
```

```sh
curl -X POST http://localhost:8000/users \
    -H "Content-Type: application/json" \
    -d '{"name":"carol"}'
```

## ドキュメント

- [seanmonstar/warp: A super-easy, composable, web server framework for warp speeds.](https://github.com/seanmonstar/warp)
- [warp docs](https://docs.rs/warp/0.3.7/warp/)
