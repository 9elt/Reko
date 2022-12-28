# User recommendations

<pre>
<a href="#user-recommendations"><b>POST</b></a> /recommendations
</pre>

## body

### *(**required**)* **`user_name`**: [string](./)  
The MyAnimeList user you want recommendations for

### *(optional)* **`accuracy`**: [number](./)  
The algorithm accuracy

### *(optional)* **`banned_ids`**: [number](./)[ ]  
An array of anime ids that will be excluded from recommendations

### *(optional)* **`banned_users`**: [string](./)[ ]  
An array of user hashes that will be excluded from recommendations

### *(optional)* **`force_list_update`**: [boolean](./)  
Syncs the user's anime list, lists older than 7 days are reloaded by default
> :warning: list update impacts significantly on response time

## responses

**`200`** *ok*

<details><summary><b>example</b></summary>

### request - *javascript*
```javascript

fetch(
    "https://api.reko.moe/recommendations",
    {
        method: 'POST',
        body: {
            user_name: "_nelt",
            accuracy: 100,
            banned_ids: [65432, 23441, 45041]
            banned_users: ["0cc175b9c0f1b6a831c399e269772661",]
            force_list_update: false,
        }
    }
)
```

### response - *json*
```json
{
    "metadata": {
        "user_last_analyzed": "2022-12-31T12:00:00.000Z",
        "users": {
            "found": 8,
            "banned": [
                "0cc175b9c0f1b6a831c399e269772661",
            ]
        },
        "rekos": {
            "sent": 24,
            "found": 107,
            "banned": [65432, 23441, 45041] 
        },
        "algorithm": {
            "passages": 1,
            "accuracy": 100,
        },
    },
    "users": [
        {
            "hash": "6asd123ff1b6a4831c399e269456dgff",
            "affinity":  88,
            "rekos": {
                "sent": 13,
                "found": 43,
            },
        },
    ],
    "rekos": [
        {
            "id": 33337,
            "info": {
                "id": 33337,
                "mean": 7.67,
                "title": "ACCA: 13-ku Kansatsu-ka",
                "airing_date": "2017-04-12",
                "main_image": "https://api.myanimelist.net/images/anime/3/83776.jpg",
                "rating": "r+",
                "num_episodes": 12,
                "genres": ["Police", "Seinen", "Action"],
                "related": [
                    {"id": 44403, "relation": "sequel"}
                ]
            },
            "expected": {
                "score":  8.23,
                "enjoyment": 95,
            },
            "users": [0, 2, 3]
        },
    ]
}
```

</details>

**`403`** *user list is private*

**`404`** *user not found*

**`429`** *rate limited*

**`5XX`** *server error*