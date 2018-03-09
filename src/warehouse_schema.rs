table! {
    audit (audit_sk) {
        audit_sk -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

table! {
    channel (channel_sk) {
        channel_sk -> Int4,
        channel_id -> Int8,
        channel_id_string -> Varchar,
        #[sql_name = "type"]
        type_ -> Int4,
        type_string -> Varchar,
        guild_id -> Nullable<Int8>,
        guild_id_string -> Nullable<Varchar>,
        guild_sk -> Int4,
        position -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        topic -> Nullable<Varchar>,
        is_nsfw -> Bpchar,
        bitrate -> Nullable<Int4>,
        user_limit -> Nullable<Int4>,
        icon -> Nullable<Bpchar>,
        owner_id -> Nullable<Int8>,
        owner_id_string -> Nullable<Varchar>,
        application_id -> Nullable<Int8>,
        application_id_string -> Nullable<Varchar>,
        parent_id -> Nullable<Int8>,
        parent_id_string -> Nullable<Varchar>,
        _effective_date -> Timestamp,
        _end_date -> Timestamp,
        _is_current -> Bool,
        _is_valid -> Bool,
        _is_deleted -> Bool,
        _is_inferred -> Bool,
        _audit_creator_sk -> Int4,
        _audit_updater_sk -> Int4,
    }
}

table! {
    date (date_sk) {
        date_sk -> Int4,
        date_version -> Int2,
        date -> Nullable<Date>,
        year -> Int4,
        month -> Int4,
        month_name -> Nullable<Varchar>,
        month_abreviation -> Nullable<Bpchar>,
        week_of_year -> Int4,
        day -> Int4,
        day_of_year -> Int4,
        weekday -> Int4,
        weekday_name -> Nullable<Varchar>,
        weekday_abreviation -> Nullable<Bpchar>,
        unix_epoch -> Nullable<Int4>,
        discord_epoch -> Nullable<Int8>,
    }
}

table! {
    emoji (emoji_sk) {
        emoji_sk -> Int4,
        is_builtin -> Bool,
        emoji_id -> Nullable<Int8>,
        emoji_id_string -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        requires_colons -> Bpchar,
        managed -> Bpchar,
        animated -> Bpchar,
        _effective_date -> Timestamp,
        _end_date -> Timestamp,
        _is_current -> Bool,
        _is_valid -> Bool,
        _is_deleted -> Bool,
        _audit_creator_sk -> Int4,
        _audit_updater_sk -> Int4,
    }
}

table! {
    guild (guild_sk) {
        guild_sk -> Int4,
        guild_id -> Int8,
        guild_id_string -> Varchar,
        name -> Nullable<Varchar>,
        icon -> Nullable<Bpchar>,
        splash -> Nullable<Bpchar>,
        region -> Nullable<Varchar>,
        owner_id -> Nullable<Int8>,
        owner_id_string -> Nullable<Varchar>,
        owner_sk -> Int4,
        permissions -> Bit,
        joined_at -> Nullable<Timestamp>,
        verification_level -> Int4,
        default_message_notifications -> Int4,
        explicit_content_filter -> Int4,
        mfa_level -> Int4,
        is_unavailable -> Bpchar,
        is_large -> Bpchar,
        member_count -> Nullable<Int4>,
        afk_timeout -> Nullable<Int4>,
        afk_channel_id -> Nullable<Int8>,
        afk_channel_id_string -> Nullable<Varchar>,
        is_embed_enabled -> Bpchar,
        embed_channel_id -> Nullable<Int8>,
        embed_channel_id_string -> Nullable<Varchar>,
        is_widget_enabled -> Bpchar,
        widget_channel_id -> Nullable<Int8>,
        widget_channel_id_string -> Nullable<Varchar>,
        application_id -> Nullable<Int8>,
        application_id_string -> Nullable<Varchar>,
        system_channel_id -> Nullable<Int8>,
        system_channel_id_string -> Nullable<Varchar>,
        _effective_date -> Timestamp,
        _end_date -> Timestamp,
        _is_current -> Bool,
        _is_valid -> Bool,
        _is_deleted -> Bool,
        _is_inferred -> Bool,
        _audit_creator_sk -> Int4,
        _audit_updater_sk -> Int4,
    }
}

table! {
    message (message_sk) {
        message_sk -> Int4,
        message_id -> Int8,
        message_id_string -> Varchar,
        channel_id -> Int8,
        channel_id_string -> Varchar,
        channel_sk -> Int8,
        author_id -> Int8,
        author_id_string -> Varchar,
        author_sk -> Int4,
        is_author_webhook -> Bool,
        content -> Nullable<Varchar>,
        timestamp -> Nullable<Timestamp>,
        edited_timestamp -> Nullable<Timestamp>,
    }
}

table! {
    presence_fact (presence_fact_pk) {
        presence_fact_pk -> Int4,
        date_sk -> Int4,
        time_sk -> Int4,
        guild_sk -> Int4,
        user_sk -> Int4,
        status -> Int4,
    }
}

table! {
    reactions_fact (fact_reaction_sk) {
        fact_reaction_sk -> Int4,
        audit_sk -> Int4,
        date_sk -> Int4,
        time_sk -> Int4,
        guild_sk -> Int4,
        channel_sk -> Int4,
        message_sk -> Int4,
        user_sk -> Int4,
        emoji_sk -> Int4,
        record_count -> Int4,
        is_removing_all_reactions -> Bool,
        is_inferred -> Bool,
    }
}

table! {
    time (time_sk) {
        time_sk -> Int4,
        time -> Nullable<Time>,
        hour -> Int4,
        minute -> Int4,
        second -> Int4,
        unix_epoch -> Nullable<Int4>,
    }
}

table! {
    user (user_sk) {
        user_sk -> Int4,
        user_id -> Int8,
        user_name -> Varchar,
        discriminator -> Int2,
        avatar -> Nullable<Bpchar>,
        bot -> Bpchar,
        mfa_enabled -> Bpchar,
        verified -> Bpchar,
        email -> Nullable<Varchar>,
        _effective_date -> Timestamp,
        _end_date -> Timestamp,
        _is_current -> Bool,
        _is_valid -> Bool,
        _is_deleted -> Bool,
        _is_inferred -> Bool,
        _audit_creator_sk -> Int4,
        _audit_updater_sk -> Int4,
    }
}

joinable!(channel -> guild (guild_sk));
joinable!(reactions_fact -> audit (audit_sk));
joinable!(reactions_fact -> channel (channel_sk));
joinable!(reactions_fact -> emoji (emoji_sk));
joinable!(reactions_fact -> guild (guild_sk));

allow_tables_to_appear_in_same_query!(
    audit,
    channel,
    date,
    emoji,
    guild,
    message,
    presence_fact,
    reactions_fact,
    time,
    user,
);
