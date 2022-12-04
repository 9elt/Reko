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
│   router.rs
│   controller.rs
└─> helper
│       └─> database
│               anime.rs
│               user.rs
│               mod.rs
│       └─> mal_api
│               anime.rs
│               list.rs
│               mod.rs
│       mod.rs
└─> models
│   │   mod.rs
│   └─> recommendations
│   └─> user_model
│           gen.rs
│           conversion.rs
│           empty.rs
│           avg.rs
│           mod.rs
└─> utils
│   │   mod.rs
│   │   mal_api.rs
│   │   time_elapsed.rs
│   └─> database
│   │       connection.rs
│   │       schema.rs
│   │       mod.rs
│   └─> converison
│           common.rs
```

</details>

