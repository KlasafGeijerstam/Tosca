table! {
    moderator (workspace_id, user_id) {
        workspace_id -> Int4,
        user_id -> Text,
    }
}

table! {
    queue (queue_id) {
        queue_id -> Int4,
        workspace_id -> Int4,
        name -> Text,
        info -> Nullable<Text>,
    }
}

table! {
    queue_slot (queue_slot_id) {
        queue_slot_id -> Int4,
        queue_id -> Int4,
        start_time -> Timestamp,
        duration -> Int4,
        open_before -> Int4,
    }
}

table! {
    queue_slot_user (queue_slot_id, user_id) {
        queue_slot_id -> Int4,
        user_id -> Text,
        message -> Nullable<Text>,
        moderator_message -> Nullable<Text>,
    }
}

table! {
    signup (signup_id) {
        signup_id -> Int4,
        workspace_id -> Int4,
        max_slot_signup -> Int4,
        name -> Text,
        info -> Nullable<Text>,
    }
}

table! {
    signup_slot (signup_slot_id) {
        signup_slot_id -> Int4,
        signup_id -> Int4,
        info -> Nullable<Text>,
        time -> Nullable<Timestamp>,
        max_users -> Int4,
    }
}

table! {
    signup_slot_user (signup_slot_id, user_id) {
        signup_slot_id -> Int4,
        user_id -> Text,
    }
}

table! {
    whitelist (workspace_id, user_id) {
        workspace_id -> Int4,
        user_id -> Text,
    }
}

table! {
    workspace (workspace_id) {
        workspace_id -> Int4,
        creator -> Text,
        name -> Text,
        info -> Nullable<Text>,
    }
}

joinable!(moderator -> workspace (workspace_id));
joinable!(queue -> workspace (workspace_id));
joinable!(queue_slot -> queue (queue_id));
joinable!(queue_slot_user -> queue_slot (queue_slot_id));
joinable!(signup -> workspace (workspace_id));
joinable!(signup_slot -> signup (signup_id));
joinable!(signup_slot_user -> signup_slot (signup_slot_id));
joinable!(whitelist -> workspace (workspace_id));

allow_tables_to_appear_in_same_query!(
    moderator,
    queue,
    queue_slot,
    queue_slot_user,
    signup,
    signup_slot,
    signup_slot_user,
    whitelist,
    workspace,
);
