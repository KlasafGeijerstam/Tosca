table! {
    group_table (gid) {
        gid -> Int4,
        creator -> Text,
        name -> Text,
        info -> Nullable<Text>,
    }
}

table! {
    moderator (gid, uid) {
        gid -> Int4,
        uid -> Text,
    }
}

table! {
    queue (qid) {
        qid -> Int4,
        gid -> Int4,
        name -> Text,
        info -> Nullable<Text>,
    }
}

table! {
    queue_slot (qsid) {
        qsid -> Int4,
        qid -> Int4,
        start_time -> Timestamp,
        duration -> Int4,
        open_before -> Int4,
    }
}

table! {
    queue_slot_user (qsid, uid) {
        qsid -> Int4,
        uid -> Text,
        message -> Nullable<Text>,
        moderator_message -> Nullable<Text>,
    }
}

table! {
    signup (sid) {
        sid -> Int4,
        gid -> Int4,
        max_slot_signup -> Int4,
        name -> Text,
        info -> Nullable<Text>,
    }
}

table! {
    signup_slot (ssid) {
        ssid -> Int4,
        sid -> Int4,
        info -> Nullable<Text>,
        time -> Nullable<Timestamp>,
        max_users -> Int4,
    }
}

table! {
    signup_slot_user (ssid, uid) {
        ssid -> Int4,
        uid -> Text,
    }
}

table! {
    whitelist (gid, uid) {
        gid -> Int4,
        uid -> Text,
    }
}

joinable!(moderator -> group_table (gid));
joinable!(queue -> group_table (gid));
joinable!(queue_slot -> queue (qid));
joinable!(queue_slot_user -> queue_slot (qsid));
joinable!(signup -> group_table (gid));
joinable!(signup_slot -> signup (sid));
joinable!(signup_slot_user -> signup_slot (ssid));
joinable!(whitelist -> group_table (gid));

allow_tables_to_appear_in_same_query!(
    group_table,
    moderator,
    queue,
    queue_slot,
    queue_slot_user,
    signup,
    signup_slot,
    signup_slot_user,
    whitelist,
);
