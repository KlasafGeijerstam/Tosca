# Tosca

* [Icons](https://github.com/tabler/tabler-icons)

## Schema

![Database structure](util/database_diagram.png)

## Foreign-API

* ValidateToken(Token) -> UserID
* GetUser(Token, UserID) -> UserData
* IsAdmin(Token, UserID) -> bool
* IsSuperUser(Token, UserID) -> bool
* GetGroupUsers(Token, GroupID) -> [UserID]
