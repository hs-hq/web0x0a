db = db.getSiblingDB('database');

db.createUser({
    user: 'root',
    pwd: 'toor',
    roles: [{ role: 'readWrite', db: 'database' }]
});

db.users.insertMany([
    {
        "username": "admin",
        "password": "$2b$10$P9JTUohy6SmqafHPdeO.D.6mLwJb97Fsz7yLLZrv/ujEpQrKzITia",
        "role": "admin"
    }
]);


db.volunteers.insertMany([
    {
        "volunteerID": 1,
        "firstName": "Jennifer",
        "lastName": "Johnson",
        "email": "jennifer@neta.com"
    },
    {
        "volunteerID": 2,
        "firstName": "Natalie",
        "lastName": "Robinson",
        "email": "natalie@neta.com"
    },
    {
        "volunteerID": 3,
        "firstName": "John",
        "lastName": "Ferguson",
        "email": "john@neta.com"
    },
    {
        "volunteerID": 4,
        "firstName": "Samuel",
        "lastName": "Patterson",
        "email": "samuel@neta.com"
    },
    {
        "volunteerID": 5,
        "firstName": "Jasmine",
        "lastName": "James",
        "email": "jasmine@neta.com"
    },
    {
        "volunteerID": 6,
        "firstName": "Patricia",
        "lastName": "Hamilton",
        "email": "patricia@neta.com"
    }
]);