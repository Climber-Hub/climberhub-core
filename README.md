ClimberHub Core
===============

This is the core of the ClimberHub project. It is an API that provides a catalog of climbing gyms and routes.  
It offers means to track your progression (mark routes as sent/flashed) and social interactions between users allowing them to comment on routes, suggest new grades, etc.  

## API Specification

Assumptions:
- The identifier of a source is stored by the core application
- The other identifers are not stored by the core application
- The identifier of a place must be unique relative to a source
- The identifier of a route must be unique relative to a place
- The identifier of a place must be a 32bit integer (or convertible to an integer)
- The identifier of a route must be a 32bit integer (or convertible to an integer)

Resources:
- Place     : a place where you can climb (gym, crag, etc.)
- Route     : a route in a gym (or outdoors)
- User      : a user of the application
- Comment   : a comment on a route
- Progress  : a user's progress on a route

Examples:
GET /sources
```json
[
    {
        "uuid": "<uuid of the source>",
        "name": "PSUC Orsay",
        "url": "https://www.psuc-orsay.fr/escalade",
    },
    {
        "uuid": "<uuid of the source>",
        "name": "Escalade 91",
        "url": "https://www.escalade91.fr/",
    }
]
```

GET /places?country=France
```json
[
    {
        "uuid": "<uuid of the place>",
        "name": "COUM",
        "description": "Centre Omnisport Universitaire de Moulon",
        "address": "8 rue 128",
        "postcode": "91190",
        "city": "Gif-sur-Yvette",
        "country": "France"
    },
    {
        "uuid": "<uuid of the place>",
        "name": "Bâtiment 225",
        "description": "Service Universitaire des Activités Physiques et Sportives Université Paris-Saclay",
        "address": "225 Rue André Ampère",
        "postcode": "91440",
        "city": "Bures-sur-Yvette",
        "country": "France",   
    }
]
```

GET /routes/:place
```json
[
    {
        "name": "Route 1",
        "description": "This is a great route",
        "grade": "6a",
        "color": "green",
        "sector": "R1",
        "rules": [
            "sitstart": false,
            "modules_allowed": false,
            "edges_allowed": false
        ],
        "tags": [
            "vertical",
            "overhang",
            "roof",
            "dihedral"
        ],
        "properties": {
            "height": "10",
            "quickdraws": "8",
        }
    }
]
```

GET /comments/:place/:route
```json
[
    {
        "username": "jdoe",
        "content": "This route is awesome !",
        "date": "2018-01-01T00:00:00Z"
    }
]
```

GET /progress/:username
```json
[
    {
        "route": "<uuid of the route>",
        "sent": true,
        "flashed": false,
        "liked": false,
        "project": true,
    }
]
```

GET /users/
```json
[
    {
        "first_name": "John",
        "last_name": "Doe",
        "username": "jdoe",
        "email": "john.doe@example.com",
    }
]
```
