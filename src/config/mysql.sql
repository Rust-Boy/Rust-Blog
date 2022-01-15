create table Pages(
    id text not null,
    title text not null,
    author text not null,
    date text not null,
    date_time text not null,
    create_time text not null,
    cove_image text not null,
    summary text not null,
    read_number text not null,
    thumb_up_number text not null,
    comment_number text not null,
    tick text not null
);

create table CommitPool(
    id text not null,
    qq text not null,
    name text not null,
    content text not null,
    date text not null
);

create table Record(
    qq text not null,
    name text not null,
    content text not null
);

create table RecordPool(
    qq text not null,
    name text not null,
    content text not null
);

create table User(
    username text not null,
    password text not null
);

create table FriendLink(
    qq text not null,
    name text not null,
    url text not null,
    descr text not null
);

create table IndexPage(
    name text not null,
    descr text not null
);