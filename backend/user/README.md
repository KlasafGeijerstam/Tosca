# Tosca user provider

Tosca is backed by a user provider. A user provider implements
a given REST-API, and provides tosca with the ability to get user permissions
and user data. 


## REST-API
The following REST-endpoints are to be implemented as part of the login and
authentication part of the provider.

### GET /users/{user\_id}
Gets user data for a user\_id received from a login provider.
Returns: A JSON object containing at least the following properties:
```json
{ 
  "user_id": "user_1",
  "first_name": "Test",
  "last_name": "Testsson",
  "user_level": 0,
  "workspaces": [
    "workspace_id1",
    "workspace_id2"
  ],
      
}
```

### GET /workspaces
Gets the available workspaces
```json
[ 
    "5DV202",
    "5DV164"
]
```

### GET /workspaces/{workspace\_id}
Get a specific workspace

```json
{
    "users": [
        { 
          "user_id": "user_1",
          "first_name": "Test",
          "last_name": "Testsson",
          "user_level": 0,
        },
        { 
          "user_id": "user_2",
          "first_name": "Test2",
          "last_name": "Testsson2",
          "user_level": 1,
        }
    ]
}
```
