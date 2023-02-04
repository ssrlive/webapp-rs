create table if not exists course (
    id bigserial NOT NULL primary key,
    teacher_id bigint not null,
    name varchar(150) not null,
    time timestamp default now(),
    description text,
    format varchar(50),
    structure text,
    duration varchar(50),
    price integer,
    language varchar(50),
    level varchar(50)
);
