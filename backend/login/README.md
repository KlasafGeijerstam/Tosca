# Tosca login

Tosca performs logins using a login provider. A login provider implements
a given REST-API, and provides a session token that can be used to receive
a user ID from the login provider.

## HTTP endpoints
The following endpoints are to be implemented as a HTTP(s) enpoint.

### GET /login
Starts the login process, examples could be that the endpoint redirects
to an OpenID Connect provider, or provides a form for the user to log in.

Configuration is specific to implementation, but response is handeled as
a redirect to a configured URL, with token and token expiry given as parameters.


## REST-API
The following REST-endpoints are to be implemented as part of the login and
authentication part of the provider.


### GET /token
Request body: `{ "token": "token_from_provider" }`

Returns a JSON object with at least the following properties:
`{ "sub": "user_id (subject)", "exp": "time in seconds before token expiry" }`

### POST /logout
Request body: `{ "token": "token_from_provider" }`

Logs out (deletes the session token) from the login server.
