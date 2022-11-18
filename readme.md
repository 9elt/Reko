# anirekome api

### development
- [x] my anime list APIs
- [X] diesel postgres setup
- [X] helpers for lists and anime
- [ ] setup axum
- [ ] db connection pool
- [ ] setup cookie based session
- [ ] model generation
- [ ] recommendations

### planned project structure

**fun.rs** files contain functions that will be called outside the module,
**structs** modules contian the structs used in the module, while **cast** modules contain
the type conversion functions.

**fetch** module handles all the data transfer and restructuring

```
src
│
│   main.rs
└─> fetch
│   │   fun.rs
│   │   mod.rs
│   └─> structs
│   │       anime.rs
│   │       list.rs
│   │       mod.rs
│   └─> cast
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