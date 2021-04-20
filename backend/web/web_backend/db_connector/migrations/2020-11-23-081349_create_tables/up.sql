-- Your SQL goes here
CREATE TABLE workspaces (
    id INT GENERATED ALWAYS AS IDENTITY,
    creator TEXT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (id)
);

CREATE TABLE whitelists (
    workspace_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, user_id),
    FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);

CREATE TABLE moderators (
    workspace_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, user_id),
    FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);

CREATE TABLE queues (
    id INT GENERATED ALWAYS AS IDENTITY,
    workspace_id INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (id),
    FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);

CREATE TABLE queue_slots (
    id INT GENERATED ALWAYS AS IDENTITY,
    queue_id INT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    duration INT NOT NULL,
    open_before INT NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    FOREIGN KEY(queue_id) REFERENCES queues(id)
);

CREATE TABLE queue_slot_users (
    queue_slot_id INT NOT NULL,
    user_id TEXT NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    moderator_message TEXT NOT NULL DEFAULT '',
    PRIMARY KEY(queue_slot_id, user_id),
    FOREIGN KEY(queue_slot_id) REFERENCES queue_slots(id)
);

CREATE TABLE signups (
    id INT GENERATED ALWAYS AS IDENTITY,
    workspace_id INT NOT NULL,
    max_slot_signup INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY(id),
    FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
);

CREATE TABLE signup_slots (
    id INT GENERATED ALWAYS AS IDENTITY,
    signup_id INT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    time TIMESTAMP,
    max_users INT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(signup_id) REFERENCES signups(id)
);

CREATE TABLE signup_slot_users (
    signup_slot_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (signup_slot_id, user_id),
    FOREIGN KEY (signup_slot_id) REFERENCES signup_slots(id)
);
