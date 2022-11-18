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
```
src
│
│   main.rs
└─> fetch
│   │   fun.rs
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