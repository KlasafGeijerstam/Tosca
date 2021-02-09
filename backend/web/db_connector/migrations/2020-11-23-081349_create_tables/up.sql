-- Your SQL goes here
CREATE TABLE workspace (
    workspace_id INT GENERATED ALWAYS AS IDENTITY,
    creator TEXT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (workspace_id)
);

CREATE TABLE whitelist (
    workspace_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, user_id),
    FOREIGN KEY(workspace_id) REFERENCES workspace(workspace_id)
);

CREATE TABLE moderator (
    workspace_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (workspace_id, user_id),
    FOREIGN KEY(workspace_id) REFERENCES workspace(workspace_id)
);

CREATE TABLE queue (
    queue_id INT GENERATED ALWAYS AS IDENTITY,
    workspace_id INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (queue_id),
    FOREIGN KEY(workspace_id) REFERENCES workspace(workspace_id)
);

CREATE TABLE queue_slot (
    queue_slot_id INT GENERATED ALWAYS AS IDENTITY,
    queue_id INT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    duration INT NOT NULL,
    open_before INT NOT NULL DEFAULT 0,
    PRIMARY KEY (queue_slot_id),
    FOREIGN KEY(queue_id) REFERENCES queue(queue_id)
);

CREATE TABLE queue_slot_user (
    queue_slot_id INT NOT NULL,
    user_id TEXT NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    moderator_message TEXT NOT NULL DEFAULT '',
    PRIMARY KEY(queue_slot_id, user_id),
    FOREIGN KEY(queue_slot_id) REFERENCES queue_slot(queue_slot_id)
);

CREATE TABLE signup (
    signup_id INT GENERATED ALWAYS AS IDENTITY,
    workspace_id INT NOT NULL,
    max_slot_signup INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    PRIMARY KEY(signup_id),
    FOREIGN KEY(workspace_id) REFERENCES workspace(workspace_id)
);

CREATE TABLE signup_slot (
    signup_slot_id INT GENERATED ALWAYS AS IDENTITY,
    signup_id INT NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    time TIMESTAMP,
    max_users INT NOT NULL,
    PRIMARY KEY(signup_slot_id),
    FOREIGN KEY(signup_id) REFERENCES signup(signup_id)
);

CREATE TABLE signup_slot_user (
    signup_slot_id INT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (signup_slot_id, user_id),
    FOREIGN KEY (signup_slot_id) REFERENCES signup_slot(signup_slot_id)
);
