# Endpoints

## GET /login

Redirects to api.42

## POST /callback

Parameters: code, state

Callback url

---

## GET /ping

Check if session is ok

## POST /evaluation

Json from the Scale_user webhook

## POST /evaluation/delete

Json from the Scale_user webhook

---

## GET /feedback/missing
### Response:
```json
[
    {
        "id": 3,
        "evaluation": {
            "team": "schibane's group-1",
            "project": "push_swap",
            "begin_at": "2023-06-21T21:30:00"
        }
    },
    {
        "id": 5,
        "evaluation": {
            "team": "schibane's group-1",
            "project": "push_swap",
            "begin_at": "2023-06-21T21:30:00"
        }
    }
]
```

## GET /feedback/{feedback_id}/info
### Response:
```json
{
    "id": 5,
    "evaluation": {
        "team": "schibane's group-1",
        "project": "push_swap",
        "begin_at": "2023-06-21T21:30:00Z",
        "correcteds": [
            "schibane"
        ],
        "corrector": "oemelyan"
    },
    "fields": [
        {
            "key": "understanding",
            "name": "The code was thoroughly understood",
            "description": "Any Questions regarding the overall structure, design choices and individual functions could be answered flawlessly.",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "uniqueness",
            "name": "The solution was unique",
            "description": "The solution provided a fresh perspective or approach that set it apart from conventional methods or existing alternatives?",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "friendliness",
            "name": "The evaluation was very pleasant",
            "description": "The atmosphere throughout the entire process was very friendly. There was no discomfort and no uneasiness.",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "comment",
            "name": "Comment",
            "description": "Optional comment you would like to share with bocal",
            "data_type": {
                "String": 1024
            }
        }
    ]
}
```

## POST /feedback/{feedback_id}
### Request
```json
{
    "understanding": 5,
    "uniqueness": 6,
    "friendliness": 7,
    "comment": "asd" // optional
}
```
