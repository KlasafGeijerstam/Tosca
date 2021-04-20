table! {
    moderators (workspace_id, user_id) {
        workspace_id -> Int4,
        user_id -> Text,
    }
}

table! {
    queue_slot_users (queue_slot_id, user_id) {
        queue_slot_id -> Int4,
        user_id -> Text,
        message -> Text,
        moderator_message -> Text,
        q_time -> Timestamp,
    }
}

table! {
    queue_slots (id) {
        id -> Int4,
        queue_id -> Int4,
        start_time -> Timestamp,
        duration -> Int4,
        open_before -> Int4,
    }
}

table! {
    queues (id) {
        id -> Int4,
        workspace_id -> Int4,
        name -> Text,
        info -> Text,
    }
}

table! {
    signup_slot_users (signup_slot_id, user_id) {
        signup_slot_id -> Int4,
        user_id -> Text,
    }
}

table! {
    signup_slots (id) {
        id -> Int4,
        signup_id -> Int4,
        info -> Text,
        time -> Nullable<Timestamp>,
        max_users -> Int4,
    }
}

table! {
    signups (id) {
        id -> Int4,
        workspace_id -> Int4,
        max_slot_signup -> Int4,
        name -> Text,
        info -> Text,
    }
}

table! {
    whitelists (workspace_id, user_id) {
        workspace_id -> Int4,
        user_id -> Text,
    }
}

table! {
    workspaces (id) {
        id -> Int4,
        creator -> Text,
        name -> Text,
        info -> Text,
    }
}

joinable!(moderators -> workspaces (workspace_id));
joinable!(queue_slot_users -> queue_slots (queue_slot_id));
joinable!(queue_slots -> queues (queue_id));
joinable!(queues -> workspaces (workspace_id));
joinable!(signup_slot_users -> signup_slots (signup_slot_id));
joinable!(signup_slots -> signups (signup_id));
joinable!(signups -> workspaces (workspace_id));
joinable!(whitelists -> workspaces (workspace_id));

allow_tables_to_appear_in_same_query!(
    moderators,
    queue_slot_users,
    queue_slots,
    queues,
    signup_slot_users,
    signup_slots,
    signups,
    whitelists,
    workspaces,
);
