-- Your SQL goes here
-- `group` is reserved
CREATE TABLE group_table (
    gid INT GENERATED ALWAYS AS IDENTITY,
    creator TEXT NOT NULL,
    name TEXT NOT NULL,
    info TEXT,
    PRIMARY KEY (gid)
);

CREATE TABLE whitelist (
    gid INT NOT NULL,
    uid TEXT NOT NULL,
    PRIMARY KEY (gid, uid),
    FOREIGN KEY(gid) REFERENCES group_table(gid)
);

CREATE TABLE moderator (
    gid INT NOT NULL,
    uid TEXT NOT NULL,
    PRIMARY KEY (gid, uid),
    FOREIGN KEY(gid) REFERENCES group_table(gid)
);

CREATE TABLE queue (
    qid INT GENERATED ALWAYS AS IDENTITY,
    gid INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT,
    PRIMARY KEY (qid),
    FOREIGN KEY(gid) REFERENCES group_table(gid)
);

CREATE TABLE queue_slot (
    qsid INT GENERATED ALWAYS AS IDENTITY,
    qid INT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    duration INT NOT NULL,
    open_before INT NOT NULL,
    PRIMARY KEY (qsid),
    FOREIGN KEY(qid) REFERENCES queue(qid)
);

CREATE TABLE queue_slot_user (
    qsid INT NOT NULL,
    uid TEXT NOT NULL,
    message TEXT,
    moderator_message TEXT,
    PRIMARY KEY(qsid, uid),
    FOREIGN KEY(qsid) REFERENCES queue_slot(qsid)
);

CREATE TABLE signup (
    sid INT GENERATED ALWAYS AS IDENTITY,
    gid INT NOT NULL,
    max_slot_signup INT NOT NULL,
    name TEXT NOT NULL,
    info TEXT,
    PRIMARY KEY(sid),
    FOREIGN KEY(gid) REFERENCES group_table(gid)
);

CREATE TABLE signup_slot (
    ssid INT GENERATED ALWAYS AS IDENTITY,
    sid INT NOT NULL,
    info TEXT,
    time TIMESTAMP,
    max_users INT NOT NULL,
    PRIMARY KEY(ssid),
    FOREIGN KEY(sid) REFERENCES signup(sid)
);

CREATE TABLE signup_slot_user (
    ssid INT NOT NULL,
    uid TEXT NOT NULL,
    PRIMARY KEY (ssid, uid),
    FOREIGN KEY (ssid) REFERENCES signup_slot(ssid)
);
