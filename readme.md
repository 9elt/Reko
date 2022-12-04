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

```
src
│   main.rs
│   router.rs
│   controller.rs
└─> models
│   │   mod.rs
│   └─> recommendations
│   │       mod.rs
│   └─> user_model
│           gen.rs
│           conversion.rs
│           empty.rs
│           avg.rs
│           mod.rs
└─> helper
│   │   mod.rs
│   └─> database
│   │       anime.rs
│   │       user.rs
│   │       mod.rs
│   └─> mal_api
│           anime.rs
│           list.rs
│           mod.rs
└─> utils
    │   mod.rs
    │   mal_api.rs
    │   time_elapsed.rs
    └─> database
    │       connection.rs
    │       schema.rs
    │       mod.rs
    └─> converison
            common.rs
```
