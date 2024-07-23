# loco playground

Rust の WEB フレームワークの Loco の検証用リポジトリ

## 構成情報

- Rust: 1.79.0
- Loco CLI: 0.2.7
- SeaORM CLI: 0.12.15
- PostgreSQL: 15.3
- Redis: 7.2.5

## ローカル環境構築

**パッケージのインストールする**

```sh
cargo install loco-cli sea-orm-cli cargo-watch
```

**開発用の環境設定ファイルを作成する**

```sh
bash tools/create_env.sh development
```

**docker compose を起動する**

```sh
docker compose up -d
```

**loco を起動する**

```sh
cargo watch -x check -s "cargo loco start"
```

### 動作確認

**疎通確認**

```sh
curl http://localhost:5150/api/_ping
```

**ヘルスチェック**

```sh
curl http://localhost:5150/api/_health
```

## UT

**テスト用の環境設定ファイルを作成する**

```sh
bash tools/create_env.sh test
```

**テスト用の docker compose を起動する**

```sh
docker compose --env-file .env.test up -d
```

**マイグレーションを実行する**

```sh
LOCO_ENV=test cargo loco db migrate
```

**初期データを登録する**

```sh
LOCO_ENV=test cargo loco task seed_data
```

**テストを実行する**

```sh
cargo test
```

## ドキュメント

- [Loco](https://loco.rs/)
- [Loco | GitHub](https://github.com/loco-rs/loco)
