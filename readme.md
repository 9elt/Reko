# anirekome api server

### development
- [x] connect my anime list apis
- [X] setup diesel postgres
- [X] helpers for lists and anime
- [ ] setup db connection pool
- [ ] setup axum
- [ ] setup cookie based session
- [ ] model generation
- [ ] recommendations

### planned project structure

Generally, **fun.rs** files contain functions that will be called outside of the module,
**structs** modules contian the structs used in the module, while **cast** modules contain
the type conversion functions and methods.

* The **fetch** module handles all the data transfering and restructuring.

```
src
│   main.rs
└─> fetch
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
└─> rec
│   └─> ...
└─> api
│   └─> ...
└─> utils
    └─> ...
```