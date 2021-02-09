# REST-API

All REST-operations must supply a session token as a http bearer token.

## Workspace

* GET `/workspaces`
* POST `/workspaces` [Super, Admin]
* GET `/workspaces/{workspace_id}`
* DELETE `workspaces/{workspace_id}` [Super, Creator]
* POST `/workspaces/{workspace_id}/moderators` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/moderators/{moderator_id}` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/whitelist/` [Super, Creator, Moderator]
* POST `/workspaces/{workspace_id}/whitelist` [Super, Creator, Moderator]
* DELETE `/workspaces/{workspace_id}/whitelist/{user_id}` [Super, Creator, Moderator]

