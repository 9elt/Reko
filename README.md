# [![Reko API](./media/logo.png)](https://reko.moe/)

> works thanks to [**MyAnimeList API *beta v2***](https://myanimelist.net/apiconfig/references/api/v2)

Reko is a Web API to match similar *MyAnimeList* users and get anime recommendations.

# How it works

Reko API matching algorithm is **hash based**.

### user hash

The hash is 64-bit and each position represents an anime statistic. The latter are ordered by mal popularity weighted for mean score.

<blockquote><small><sup>example</sup></small>

***`0`** 12 Episodes series, **`1`** Action genre, **`2`** Fantasy genre,  **. . .** **`6`** Aired in the 20s, **. . .** **`59`** PG rating **. . .***
</blockquote>

A user hash is generated from his list, using the statistics of the last 256 anime he's watched. Each bit is assigned `1` when the corresponding stat has `times watched * scores` greater than the next one.

In few words, the hash contains information on what the user **watches and scores more** than average. Similar users generate similar hashes and vice versa.

### user comparison

Users are compared by computing the [***hamming distance***](https://en.wikipedia.org/wiki/Hamming_distance) on their hashes, adding the *hamming distance* of the first 48-bits.

The result is a number between 0 and 112, where, 0 is 100% similarity and 112 is 0%.

### user recommendations

Anime recommendations are taken from the 32 most similar users, and ordered by popularity and score among those users.

# Documentation

The Reko API docs are available [**here**](https://reko.moe/docs).
