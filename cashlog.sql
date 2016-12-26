
create sequence account_seq;

-- User account.
create table account (
    id bigint primary key,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);


create sequence account_email_seq;

-- Account's email. Each account can potentially have
-- many emails. Email can be used to sign in into account.
create table account_email (
    id bigint primary key,
    account bigint not null references account,
    email varchar(128) not null,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);

create unique index account_email_ui on account_email (email);


create sequence login_token_seq;

-- User that knows the token can access the account.
-- Tokens can be sent via email, be embedded in links etc.
-- Tokens are destroyed after use (marked as used).
-- Tokens have expiry date.
create table login_token (
    id bigint primary key,
    account bigint not null references account,
    token varchar(128) not null,
    used boolean not null,
    used_ts timestamp without time zone,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);

-- Login tokens are unique, never repeat.
create unique index login_token_token_ui on login_token (token);


create sequence session_seq;

-- Arbitrary session k-v.
-- Session to account link is currently stored in the account key for given
-- session, and the value is the digit string id of account.
create table session (
    id bigint primary key,
    key varchar(128) not null,
    name varchar(128) not null,
    value varchar(1024) not null,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);

-- In each session (identified by key) there's only one value by given
-- name.
create unique index session_key_name_i on session (key, name);


create sequence entry_seq;

-- The cashlog entry.
create table entry (
    id bigint primary key,
    account bigint not null references account,
    bank_account varchar(32) not null,
    ts timestamp without time zone not null,
    amount numeric not null,
    currency varchar(3) not null,
    deleted bool not null default false,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);

-- Most of the time we'll be querying by user account, and
-- usually ordered.
create index entry_bank_account_ts_i on entry (bank_account, ts);
