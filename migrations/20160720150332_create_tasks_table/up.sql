create table tasks (
    id serial primary key,
    description text not null,
    completed boolean not null default false
);

insert into tasks (description) values ('demo task');
insert into tasks (description) values ('demo task2');
