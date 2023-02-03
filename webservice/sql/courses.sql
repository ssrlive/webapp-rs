create table if not exists courses (
    id bigserial NOT NULL primary key,
    teacher_id bigint not null,
    name varchar(150) not null,
    time timestamp default now() not null
);
