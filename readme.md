# anirekome api server

### Development
- [x] connect my anime list apis
- [X] setup diesel postgres
- [X] helpers for lists and anime
- [X] setup axum
- [X] base model generation
- [X] model db table
- [X] setup db connection pool
- [X] average model
- [ ] jsonb queries
- [ ] model affinity alorithm
- [ ] recommendations

### Project structure

Generally, **fun.rs** files contain functions that will be called outside of the module,
**structs** modules contian the structs used in the module, while **cast** modules contain
the type conversion functions and methods.

* The **api** module handles requests to the server.
* The **model** module handles the user model generation.
* The **data** module handles data transfering and restructuring.

<details><summary><b>tree</b></summary>

```
src
│   main.rs
└─> api
│       controller.rs
│       router.rs
│       mod.rs
└─> model
│   │   base.rs
│   │   mod.rs
│   └─> cast
│           base.rs
│           mod.rs
└─> data
│   │   fun.rs
│   │   mod.rs
│   └─> structs
│   │       anime.rs
│   │       list.rs
│   │       mod.rs
│   └─> cast
│   │       generic.rs
│   │       anime.rs
│   │       list.rs
│   │       mod.rs
│   └─> MAL
│   │       fun.rs
│   │       headers.rs
│   │       mod.rs
│   └─> database
│           fun.rs
│           connection.rs
│           schema.rs
│           mod.rs
└─> utils
    └─> ...
```

</details>

