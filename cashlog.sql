
create sequence entry_seq;

create table entry (
    id integer primary key,
    ts timestamp without time zone not null,
    amount numeric not null,
    currency varchar(3) not null
);
