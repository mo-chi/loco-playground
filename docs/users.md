# ユーザ操作 リクエスト

**ユーザ登録**

```sh
curl -X POST http://localhost:5150/api/auth/register \
     -H "Content-Type: application/json" \
     --data-raw '{
         "name": "Loco user",
         "email": "user@loco.rs",
         "password": "12341234"
     }'
```

**ログイン**

```sh
curl -s -X POST http://localhost:5150/api/auth/login \
     -H "Content-Type: application/json" \
     --data-raw '{
         "email": "user@loco.rs",
         "password": "12341234"
     }' | jq
```

**リクエストユーザの確認**

```sh
curl http://localhost:5150/api/me \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer TOKEN"
```
