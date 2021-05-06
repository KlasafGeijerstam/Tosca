-- This file should undo anything in `up.sql`
ALTER TABLE workspaces
    RENAME COLUMN creator_user_id TO creator;
