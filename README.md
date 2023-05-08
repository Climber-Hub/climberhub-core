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
GET /places?country=France
```json
[
    {
        "id"          : "1",
        "name"        : "COUM",
        "description" : "Centre Omnisport Universitaire de Moulon",
        "address"     : "8 rue 128",
        "postcode"    : "91190",
        "city"        : "Gif-sur-Yvette",
        "country"     : "France"
    },
    {
        "id"          : "2",
        "name"        : "Bâtiment 225",
        "description" : "Service Universitaire des Activités Physiques et Sportives Université Paris-Saclay",
        "address"     : "225 Rue André Ampère",
        "postcode"    : "91440",
        "city"        : "Bures-sur-Yvette",
        "country"     : "France",   
    }
]
```
[See schema](schemas/place/schema.json)

GET /routes/
```json
[
    {
        "id"          : "1-1",
        "place_id"    : "1",
        "name"        : "Route 1",
        "description" : "This is a great route",
        "grade"       : "6a",
        "color"       : "green",
        "sector"      : "R1",
        "rules"       : [
            "sitstart"        : false,
            "modules_allowed" : false,
            "edges_allowed"   : false
        ],
        "opening_date" : "2021-01-01T10:30:00Z",
        "closing_date" : null,
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
[See schema](schemas/route/schema.json)

GET /comments/:place/:route
```json
[
    {
        "username" : "jdoe",
        "content"  : "This route is awesome !",
        "date"     : "2018-01-01T00:00:00Z"
    }
]
```
[See schema](schemas/comment/schema.json)

GET /progress/:username
```json
[
    {
        "route_id" : "3-15",
        "sent"     : true,
        "flashed"  : false,
        "liked"    : false,
        "project"  : true
    }
]
```
[See schema](schemas/progress/schema.json)

GET /users/
```json
[
    {
        "first_name" : "John",
        "last_name"  : "Doe",
        "username"   : "jdoe",
        "email"      : "john.doe@example.com"
    }
]
```
[See schema](schemas/user/schema.json)
