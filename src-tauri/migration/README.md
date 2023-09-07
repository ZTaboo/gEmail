## new table

```shell
sea-orm-cli migrate generate template -u "sqlite://C://Users//zero//.config//g_email//g_email.db"
```

## run migration

```shell
sea-orm-cli migrate refresh -u "sqlite://C://Users//zero//.config//g_email//g_email.db"
```

## gen entity
```shell
sea-orm-cli generate entity -u "sqlite://./g_email.db" --with-serde serialize -o src\entity
```