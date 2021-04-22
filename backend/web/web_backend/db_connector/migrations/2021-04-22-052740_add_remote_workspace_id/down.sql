-- This file should undo anything in `up.sql`
ALTER TABLE workspaces
    DROP COLUMN remote_workspace_id;
