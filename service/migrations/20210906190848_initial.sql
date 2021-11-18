create table broadcasters
(
    id            bigint       not null primary key,
    login         varchar(255) not null,
    display_name  varchar(255) not null,
    profile_image varchar(255),
    color         varchar(255)
);

create table games
(
    id      bigint       not null primary key,
    name    varchar(255) not null,
    display varchar(255) not null,
    boxart  varchar(255)
);

insert into games (id, name, display)
values (0, 'Unknown', 'Unknown');

create table tags
(
    id              uuid         not null primary key,
    is_language_tag boolean      not null default 'false',
    localized_name  varchar(255) not null,
    name            varchar(255) not null
);

create table streams
(
    id            bigint not null primary key,
    title         varchar(255),
    preview_image varchar(255),
    broadcaster   bigint,
    game          bigint,
    tags          uuid[] not null default '{}',
    stream_type   varchar(255),
    updated_at    timestamp default current_timestamp,
    foreign key (broadcaster) references broadcasters (id),
    foreign key (game) references games (id)
);

create
index tags_index on streams using gin (tags);


