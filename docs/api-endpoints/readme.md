[Docs](../) > [API endpoints](./)

### content
* [User recommendations](#user-recommendations)
* [User statistics](#lock-user-statistics)

# User recommendations

<pre>
<a href="#user-recommendations"><b>POST</b></a> /recommendations/{myanimelist_username}
</pre>

## post data

### *(optional)* **reload_list**: [boolean](./)  
Sync the user's anime list, lists older than 7 days are reloaded by default

> :warning: list reloading impacts significantly on response time

### *(optional)* **banned_ids**: [number](./)[ ] 
An array of anime ids that will be excluded from recommendations

## responses

**200** OK

<details><summary>Example</summary>

### request - *javascript*
```javascript

fetch(
    "https://api.reko.moe/recommendations",
    {
        method: 'POST',
        body: {
            user_name: "_nelt",
            reload_list: false,
            banned_ids: [65432, 23441, 45041]
            banned_users: ["85136c79cbf9fakl2j5d0639c70c265c18d37"]
        }
    }
)
```

### response - *json*
```json
{
    "metadata": {
        "user_last_analyzed": "2022-12-31T12:00:00.000Z",
        "users_found": 8,
        "rekos": {
            "sent": 24,
            "found": 107
        },
        "algorithm": {
            "passages": 1,
            "accuracy": 100,
        },
    },
    "users": [
        {
            "hash": "85136c79cbf9fe36bb9d05d0639c70c265c18d37",
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

**403** PRIVATE USER LSIT

**404** USER NOT FOUND

**429** RATE LIMITED

**5XX** SERVER ERROR

# :lock: User statistics
Even though these statistics are generated out of publicly available data, they are quite detailed, and could potentially reveal some delicate information about the user (such as age, gender, sexual orientation, etc.), hence this endpoint is private and requires an [**authentication key**](./)
<pre>
<a href="#lock-user-statistics"><b>GET</b></a> /stats/{myanimelist_username}
</pre>

## query parameters

### *(optional)* **reload_list**: [boolean](./)
Sync the user's anime list, lists older than 7 days are reloaded by default

> :warning: list reloading impacts significantly on response time

## responses

**200** OK

**403** PRIVATE USER LSIT

**404** USER NOT FOUND

**5XX** SERVER ERROR
