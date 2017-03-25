
create sequence bank_account_seq;

create table bank_account (
    id bigint primary key,
    account bigint not null references account,
    name varchar(32) not null,
    currency varchar(3) not null,
    deleted bool default false,
    created timestamp without time zone not null,
    modified timestamp without time zone not null
);

create unique index bank_account_account_name_currency_i
on bank_account (account, name, currency);

insert into bank_account (
    id,
    account,
    name,
    currency,
    deleted,
    created,
    modified
)
select
    nextval('bank_account_seq'),
    eba.account,
    eba.name,
    eba.currency,
    eba.deleted,
    current_timestamp,
    current_timestamp
from
    (
        select
            entry.account,
            entry.bank_account as name,
            entry.currency,
            case
                -- sum of live entries
                sum(
                    case entry.deleted
                    when false then 1
                    else 0
                    end
                )
                -- when sum of live entries is zero, then bank account is deleted
                when 0 then true
                else false
            end as deleted
        from entry
        group by
            entry.account,
            entry.bank_account,
            entry.currency
    ) as eba;

alter table entry add column bank_account_id bigint references bank_account;

update entry
set bank_account_id = (
    select bank_account.id
    from bank_account
    where
        bank_account.account = entry.account
        and bank_account.name = entry.bank_account
        and bank_account.currency = entry.currency
);

alter table entry drop column bank_account;

alter table entry rename column bank_account_id to bank_account;

alter table entry drop column account;

alter table entry drop column currency;

