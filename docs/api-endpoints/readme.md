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

### *(optional)* **show_names**: [boolean](./)  
Show recommending users' usernames
> :lock: requires an [**authentication key**](./)

### *(optional)* **banned_ids**: [number](./)[ ] 
An array of anime ids that will be excluded from recommendations

## responses

**200** OK

<details><summary>Example</summary>

### request - *javascript*
```javascript
fetch(
    "https://api.reko.moe/recommendations/_nelt",
    {
        method: 'POST',
        body: JSON.stringify({
            reload_list: false,
            show_names: false,
            anime_to_ignore: [65432, 23441, 45041]
        })
    }
)
```

### response - *json*
```json
{
    "user": {
        "user_name": "_nelt",
        "list_last_update": "2022-12-31T12:00:00.000Z",
        "recommendation_accuracy": 100,
        "recommending_users_found": 3,
    },
    "recommending_users": [
        {
            "user_name": "85136c79cbf9fe36bb9d05d0639c70c265c18d37",
            "affinity": {
                "value": 88,
                "std_deviation": 4.03
            }
        },
        {
            "user_name": "b5cc17d3a35877ca8b76f0b2e07497039c250696",
            "affinity": {
                "value": 67,
                "std_deviation": 3.39
            }
        },
        {
            "user_name": "d5089e60f7bdc4ef2ac71ca06ee5d79c3fd3f328",
            "affinity": {
                "value": 98,
                "std_deviation": 12.54
            }
        },
    ],
    "recommendations": [
        {
            "id": 33337,
            "title": "ACCA: 13-ku Kansatsu-ka",
            "image": "https://api.myanimelist.net/images/anime/3/83776.jpg",
            "mean": 7.67,
            "stats": {
                "expected_score": {
                    "value": 8.23,
                    "std_deviation": 1.25,
                },
                "affinity": {
                    "value": 88,
                    "std_deviation": 6.03,
                },
            },
            "recommending_users": [0, 2]
        },
        {
            "id": 33337,
            "title": "ACCA: 13-ku Kansatsu-ka",
            "image": "https://api.myanimelist.net/images/anime/3/83776.jpg",
            "mean": 7.67,
            "stats": {
                "expected_score": {
                    "value": 8.23,
                    "std_deviation": 1.25,
                },
                "affinity": {
                    "value": 88,
                    "std_deviation": 6.03,
                },
            },
            "recommending_users": [0, 2]
        }
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
