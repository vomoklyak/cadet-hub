create table if not exists users (
    id INTEGER primary key AUTOINCREMENT,
    login TEXT not null unique,
    password TEXT not null,
    role TEXT not null,
    created_at INTEGER DEFAULT (strftime('%s', 'now')) not null,
    updated_at INTEGER DEFAULT (strftime('%s', 'now')) not null
);
INSERT INTO users(id, login, password, role) VALUES(1,'admin','$argon2i$v=19$m=16,t=2,p=1$eVlWS2d3MmtPVDFNSnZPag$7O7zCguy5+n5LxW4G7Oi9A','Admin');


create table if not exists cadets (
    id INTEGER primary key AUTOINCREMENT,
    tax_number TEXT not null unique,
    first_name TEXT not null,
    middle_name TEXT not null,
    last_name TEXT not null COLLATE NOCASE,
    birth_date INTEGER not null,
    created_at INTEGER DEFAULT (strftime('%s', 'now')) not null,
    updated_at INTEGER DEFAULT (strftime('%s', 'now')) not null
);
create index if not exists idx_cadet_tax_number on cadets(tax_number);
create index if not exists idx_cadet_last_name on cadets(last_name);
create index if not exists idx_cadet_birth_date on cadets(birth_date);


create table if not exists cadet_courses (
     id INTEGER primary key AUTOINCREMENT,
     cadet_id INTEGER not null,
     military_rank TEXT not null,
     source_unit TEXT not null,
     specialty_name TEXT not null,
     specialty_code TEXT not null,
     specialty_mos_code TEXT not null,
     category TEXT not null,
     training_location TEXT not null,
     start_date INTEGER not null,
     end_date INTEGER not null,
     completion_order_number TEXT not null,
     completion_certificate_number TEXT not null,
     notes TEXT,
     created_at INTEGER DEFAULT (strftime('%s', 'now')) not null,
     updated_at INTEGER DEFAULT (strftime('%s', 'now')) not null,

     foreign key (cadet_id) references cadets (id) on delete cascade
     unique (cadet_id, specialty_code, end_date)
);
create index if not exists idx_start_date on cadet_courses(start_date);
create index if not exists idx_end_date on cadet_courses(end_date);