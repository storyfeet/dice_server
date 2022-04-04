
Memory:
    Logins(key_to_dash):
        key,
        expiry,
        username,
        

Databases:
    Users(Username):
        Password,

    Guests(Username):
        List(User):
            permissions 

    Events(Username/Roomname):
        List: Event

    Status(Username/Roomname):
        List: Room Items

    Templates(PATH): //Classes
        <TODO>
    Items: //Instances
        <Template Contents>
    Resources:
        Blob file data
    Rooms : Event Logs


