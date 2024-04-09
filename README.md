# [Reko API](https://reko.moe/)

Reko is a Web API to match similar [***MyAnimeList***](https://myanimelist.net) users and get anime recommendations. Works thanks to [*MyAnimeList API *beta v2**](https://myanimelist.net/apiconfig/references/api/v2).

## Algorithm

Reko matching algorithm is **hash based**, it uses a 64-bit hash with each position representing an anime statistic. The latter are ordered by mal popularity weighted for mean score.

<blockquote><small><sup>complete <a href="statistics.md">reference</a>, example:</sup></small>

***`0`** PG-13 rating, **`1`** 12 Episodes series, **`2`** Aired in the 10s,  **`3`** Action genre, **. . .** **`63`** RX rating*
</blockquote>

The hash is generated from a user list, using the statistics of the last 256 anime he's watched. Each bit is assigned `1` when the corresponding stat has `times watched * scores` greater than the next one.

In few words, the hash roughly stores information on what the user **watches and scores** more than average. Similar users generate similar hashes and vice versa.

### user comparison

Users are compared by computing the [***hamming distance***](https://en.wikipedia.org/wiki/Hamming_distance) on their hashes.

Since genres and themes use most of the hash space (45 bits) the *hamming distance* is computed twice, first on the entire hash, then, with a mask that removes genres and themes bits.

The result is a number between 0 and 83, where 0 is 100% similarity and 83 is 0%.

### anime recommendations

Anime recommendations are taken from other users lists.

Recommendations from the most similar users lists are ordered by the following function:

```
entry scores / (entry count + 1) - floor(avg. years from entry update / 2)
```

Recommendations from specific users are ordered by:

```
entry score - floor(years from entry update / 2)
```

## Documentation

The Reko API docs are available [**here**](https://reko.moe/docs).
