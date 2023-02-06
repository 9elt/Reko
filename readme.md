[![Reko API](https://github.com/9elt/Reko/blob/readme-media/reko-logo.png)](https://reko.moe/)

Reko is a Web API that provides **anime recommendations** to **MyAnimeList** users, by matching them with other similar users.


> works thanks to [MyAnimeList API *beta v2*](https://myanimelist.net/apiconfig/references/api/v2)

#### index
* [statistics model](#statistics-model)
* [normal distribution](#normal-distribution)
* [recommendations](#recommendations)
* [project structure](#project-structure)

# [statistics model](#statistics-model)
> `src` / [**`algorithm`**](src/algorithm) / [**`user`**](src/algorithm/user) / [**`stats.rs`**](src/algorithm/user/stats.rs)

A model of the user is **generated from his anime list**, the model includes:

### [statistics types](#statistics-types)

Overall
* **general** `general`

Detailed
* **airing decades**: `1980s` `1990s` `2000s` `2010s` `2020s`
* **series lengths**: `1 episode` `2-8 episodes` `9-18 episodes` `19-32 episodes` `33+ episodes`
* **ratings**: `g` `pg` `pg-13` `r` `r+` `rx`
* **major genres**: The **8** most common genres (e.g. `Action` `Fantasy` `Romance`)
* **minor genres**: The other **13** genres (e.g. `Horror` `Slice of Life` `Sports`)
* **major themes**: The **20** most common themes (e.g. `Isekai` `Historical` `School`)
* **minor themes**: The other **30** themes (e.g. `Detective` `Showbiz` `Otaku Culture`)
* **demographics**: `Kids` `Shounen` `Seinen` `Josei` `Shoujo`

<blockquote><details><summary>all <i>MyAnimeList</i> themes and genres</summary>

#### MAJOR Genres
[**`Action`**](https://myanimelist.net/anime/genre/1) [**`Adventure`**](https://myanimelist.net/anime/genre/2) [**`Comedy`**](https://myanimelist.net/anime/genre/4) [**`Drama`**](https://myanimelist.net/anime/genre/8) [**`Fantasy`**](https://myanimelist.net/anime/genre/10) [**`Romance`**](https://myanimelist.net/anime/genre/22) [**`Sci Fi`**](https://myanimelist.net/anime/genre/24) [**`Supernatural`**](https://myanimelist.net/anime/genre/37)

#### minor Genres
[**`Avant Garde`**](https://myanimelist.net/anime/genre/5) [**`Award Winning`**](https://myanimelist.net/anime/genre/46) [**`Boys Love`**](https://myanimelist.net/anime/genre/28) [**`Girls Love`**](https://myanimelist.net/anime/genre/26) [**`Gourmet`**](https://myanimelist.net/anime/genre/47) [**`Horror`**](https://myanimelist.net/anime/genre/14) [**`Mystery`**](https://myanimelist.net/anime/genre/7) [**`Slice of Life`**](https://myanimelist.net/anime/genre/36) [**`Sports`**](https://myanimelist.net/anime/genre/30) [**`Suspense`**](https://myanimelist.net/anime/genre/41) [**`Ecchi`**](https://myanimelist.net/anime/genre/9) [**`Erotica`**](https://myanimelist.net/anime/genre/49) [**`Hentai`**](https://myanimelist.net/anime/genre/12) 

#### MAJOR Themes
[**`Adult Cast`**](https://myanimelist.net/anime/genre/50) [**`Gag Humor`**](https://myanimelist.net/anime/genre/57) [**`Gore`**](https://myanimelist.net/anime/genre/58) [**`Harem`**](https://myanimelist.net/anime/genre/35) [**`Historical`**](https://myanimelist.net/anime/genre/13) [**`Isekai`**](https://myanimelist.net/anime/genre/62) [**`Iyashikei`**](https://myanimelist.net/anime/genre/63) [**`Love Polygon`**](https://myanimelist.net/anime/genre/64) [**`Martial Arts`**](https://myanimelist.net/anime/genre/17) [**`Mecha`**](https://myanimelist.net/anime/genre/18) [**`Military`**](https://myanimelist.net/anime/genre/38) [**`Music`**](https://myanimelist.net/anime/genre/19) [**`Mythology`**](https://myanimelist.net/anime/genre/6) [**`Parody`**](https://myanimelist.net/anime/genre/20) [**`Psychological`**](https://myanimelist.net/anime/genre/40) [**`School`**](https://myanimelist.net/anime/genre/23) [**`Super Power`**](https://myanimelist.net/anime/genre/31) [**`Survival`**](https://myanimelist.net/anime/genre/76) [**`Time Travel`**](https://myanimelist.net/anime/genre/78) [**`Vampire`**](https://myanimelist.net/anime/genre/32)   

#### minor Themes
[**`Anthropomorphic`**](https://myanimelist.net/anime/genre/51) [**`CGDCT`**](https://myanimelist.net/anime/genre/52) [**`Childcare`**](https://myanimelist.net/anime/genre/53) [**`Combat Sports`**](https://myanimelist.net/anime/genre/54) [**`Crossdressing`**](https://myanimelist.net/anime/genre/81) [**`Delinquents`**](https://myanimelist.net/anime/genre/55) [**`Detective`**](https://myanimelist.net/anime/genre/39) [**`Educational`**](https://myanimelist.net/anime/genre/56) [**`High Stakes Game`**](https://myanimelist.net/anime/genre/59) [**`Idols Female`**](https://myanimelist.net/anime/genre/60) [**`Idols Male`**](https://myanimelist.net/anime/genre/61) [**`Magical Sex Shift`**](https://myanimelist.net/anime/genre/65) [**`Mahou Shoujo`**](https://myanimelist.net/anime/genre/66) [**`Medical`**](https://myanimelist.net/anime/genre/67) [**`Organized Crime`**](https://myanimelist.net/anime/genre/68) [**`Otaku Culture`**](https://myanimelist.net/anime/genre/69) [**`Performing Arts`**](https://myanimelist.net/anime/genre/70) [**`Pets`**](https://myanimelist.net/anime/genre/71) [**`Racing`**](https://myanimelist.net/anime/genre/3) [**`Reincarnation`**](https://myanimelist.net/anime/genre/72) [**`Reverse Harem`**](https://myanimelist.net/anime/genre/73) [**`Romantic Subtext`**](https://myanimelist.net/anime/genre/74) [**`Samurai`**](https://myanimelist.net/anime/genre/21) [**`Showbiz`**](https://myanimelist.net/anime/genre/75) [**`Space`**](https://myanimelist.net/anime/genre/29) [**`Strategy Game`**](https://myanimelist.net/anime/genre/11) [**`Team Sports`**](https://myanimelist.net/anime/genre/77) [**`Video Game`**](https://myanimelist.net/anime/genre/79) [**`Visual Arts`**](https://myanimelist.net/anime/genre/80) [**`Workplace`**](https://myanimelist.net/anime/genre/48) 

<br>

*buttons link to respective MyAnimeList genre/theme page*
</details></blockquote>

### [statistics](#statistics)

Each statistic (e.g. `2010s`, `r+`, `Romance`) includes:
* **percentage of entries** in list (or list length on `general`)
* average **MAL mean** score
* average user **score deviation**
* average user **scored percentage**
* average of every **status** percentage

> statuses are: `completed` `plan to watch` `watching` `on hold` `dropped`

# [normal distribution](#normal-distribution)
> `src` / [**`algorithm`**](src/algorithm) / [**`analysis.rs`**](src/algorithm/analysis.rs)

A job periodically calculates the **mean** and **standard deviation** of each statistic in the **[statistics model](#statistics-model)**, from all the users in the database.

# [recommendations](#recommendations)

### [finding similar users](#finding-similar-users)
> `src` / [**`algorithm`**](src/algorithm) / [**`user`**](src/algorithm/user) / [**`affinity.rs`**](src/algorithm/user/affinity.rs)

A minimum of 4 similar users are found by querying the database for users that **deviate by less than `X`%** on each statistic of the **[statistics model](#statistics-model)** 

`X` value varies on the different types of statistics, and increments when no users are found.
<details><summary>The values of <code>X</code> table</summary>

| value of `X`   | perc | mean score | score dev | scored perc | ...statuses |
|----------------|------|------------|-----------|-------------|-------------|
| general        |      | 2.5c       | 8c        | 8c          | 8c          |
| airing decades | 3c   | 7.5c       | 24c       | 24c         | 24c         |
| series length  | 3c   | 7.5c       | 24c       | 24c         | 24c         |
| ratings        | 3c   | 7.5c       | 24c       | 24c         | 24c         |
| major genres   | 4c   | 10c        | 32c       | 32c         | 32c         |
| minor genres   | 5c   | 12.5c      | 40c       | 40c         | 40c         |
| major themes   | 4c   | 10c        | 32c       | 32c         | 32c         |
| minor themes   | 5c   | 12.5c      | 40c       | 40c         | 40c         |
| demographics   | 4c   | 10c        | 32c       | 32c         | 32c         |

**`c`** = **250,000** / **number of users** in the database
</details>

### [extracting recommendations](#extracting-recommendations)
> `src` / [**`algorithm`**](src/algorithm) / [**`user`**](src/algorithm/user) / [**`recommendation.rs`**](src/algorithm/user/recommendation.rs)

Entries that have:
* status of `completed` or `watching` 
* at least **1 episode** watched
* **no prequel or parent story**

are pulled out of [similar users](#finding-similar-users) lists, ordered by their [expected enjoyment score](#expected-enjoyment-score) and finally sent as recommendations.

### [expected enjoyment score](#expected-enjoyment-score)
> `src` / [**`algorithm`**](src/algorithm) / [**`user`**](src/algorithm/user) / [**`recommendation.rs`**](src/algorithm/user/recommendation.rs)

An anime **expected enjoyment score** is calculated by averaging the user watched percentage deviation from the mean of each statistic of the anime

#### example

An anime was aired in the `2010s`, has `~12 episodes`, an `r+` rating, `Action` and `Isekai` genres.

The user watched percentage deviation is: 
* `2010s` => 9%
* `~12 episodes` => 29%
* `r+` => -3%
* `Action` => 2%
* `Isekai` => 12%

the enjoyment score will be:
```
(9 + 29 -3 + (2 + 12) / 2) / 4 = 12.25
```

# [project structure](#project-structure)

<pre>
<a href="src/">src</a>
│   <a href="src/main.rs">main.rs</a>
│   <a href="src/router.rs">router.rs</a> <i>...API router</i>
│
└─> <a href="src/controllers">controllers</a>
│     <i>...API controllers</i>
│
└─> <a href="src/models">models</a>
│     <i>...API models</i>
│
└─> <a href="src/algorithm">algorithm</a>
|   │   <a href="src/algorithm/analysis.rs">analysis.rs</a> <i>...normal distribution calculation</i>
│   └─> <a href="src/algorithm/model">model</a>
│   │     <i>...statistics model struct and helpers</i>
│   └─> <a href="src/algorithm/user">user</a>
│         <i>...user statistics, recommendations etc..</i>
│
└─> <a href="src/helper">helper</a>
│     <i>...database and MyAnimeList API data fetching</i>
│
└─> <a href="src/utils">utils</a>
      <i>...db connections, API headers, etc...</i>
</pre>

# dev to dos
- [ ] helper refactory (optimize conversions etc)  
- [ ] implement logger
- [ ] implement rate limiter
- [ ] solve the prequel problem
