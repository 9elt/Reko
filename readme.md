# Anirekome APIs

**Take an anime list** and returns anime **recommendations** from the lists of **similar users**.

> works thanks to [*myanimelist.net* beta v2 API](https://myanimelist.net/apiconfig/references/api/v2)

#### index
* [statistics model](#statistics-model)
* [recommendation](#recommendation)
* [project structure](#project-structure)

# [statistics model](#statistics-model)
> `src` / `algorithm` / [**`stats.rs`**](src/algorithm/model/stats.rs)

A model of the user is **generated from his anime list**, the model includes:

### [general statistics](#general-statistics)

General statistics include:
* list length
* average **MAL mean** score
* average **score deviation**
* average **scored percentage**
* average of every **status** percentage

> statuses are: `completed` `plan to watch` `watching` `on hold` `dropped`

### [detailed statistics](#detailed-statistics)

Detailed statistics are:
* **airing decades**: `1980s` `1990s` `2000s` `2010s` `2020s`
* **series lengths**: `1 episode` `2-8 episodes` `9-18 episodes` `19-32 episodes` `33+ episodes`
* **ratings**: `g` `pg` `pg-13` `r` `r+` `rx`
* **major genres**: The **8** most common genres (`Action` `Fantasy` `Romance`)
* **minor genres**: The **13** other genres (`Horror` `Slice of Life` `Sports`)
* **major themes**: The **20** most common themes (`Isekai` `Historical` `School`)
* **minor themes**: The **30** other themes (`Detective` `Showbiz` `Otaku Culture`)
* **demographics**: `Kids` `Shounen` `Seinen` `Josei` `Shoujo`

<blockquote><details><summary>all <i>myanimelist.net</i> themes and genres</summary>

#### MAJOR Genres
[**`Action`**](/) `Fantasy` `Romance`

#### minor Genres
`Horror` `Slice of Life` `Sports`

#### MAJOR Themes
`Isekai` `Historical` `School`

#### minor Themes
`Detective` `Showbiz` `Otaku Culture`

<br>

*buttons link to respective myanimelist.net genre/theme page*
</details></blockquote>


Each detailed statistic includes:
* statistic overall percentage
* average **MAL mean** score
* average **score deviation**
* average **scored percentage**
* average of every **status** percentage

# [recommendation](#recommendation)

### [finding similar users](#finding-similar-users)
> `src` / `algorithm` / [**`affinity.rs`**](src/algorithm/model/affinity.rs)

First of all we query the database for users that **deviate by less than `X`%** on **[general statistics](#general-statistics)** and the distirbution on **[detailed statistic](#detailed-statistics)** deviates by less than **`X`%**

`X` value varies on the different types of statistics
<details><summary>The values of <code>X</code> table</summary>

| value of `X`   | perc | mean score | score dev | scored perc | completed | plan to watch | watching | on hold | dropped |
|----------------|------|------------|-----------|-------------|-----------|---------------|----------|---------|---------|
| general        |      | 1c         | 2c        | 2c          | 2c        | 3c            | 3c       | 3c      | 3c      |
| airing decades | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| series length  | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| ratings        | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| major genres   | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| minor genres   | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| major themes   | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| minor themes   | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |
| demographics   | 2c   | 2c         | 4c        | 4c          | 6c        | 8c            | 8c       | 8c      | 6c      |

with **`c`** = **500,000** / **number of users** in the database
</details>

# [project structure](#project-structure)

<pre>
<a href="src/">src</a>
│   main.rs
│   router.rs <i>...api router</i>
│   controller.rs <i>...api controller</i>
│
└─> <a href="src/models">models</a>
│     <i>...api models</i>
│
└─> <a href="src/algorithm">algorithm</a>
│   └─> <a href="src/algorithm/mean">mean</a>
│   │     <i>...mean models calculation and storage</i>
│   └─> <a href="src/algorithm/user">user</a>
│         <i>...user recommendations</i>
│
└─> <a href="src/helper">helper</a>
│   └─> <a href="src/helper/database">database</a>
│   │     <i>...database data fetching</i>
│   └─> <a href="src/helper/mal_api">mal_api</a>
│         <i>...myanimelist api data fetching</i>
│
└─> <a href="src/utils">utils</a>
      <i>...db connections, api headers, etc...</i>
</pre>
