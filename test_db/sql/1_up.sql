create table ingredients (
	id uuid primary key,
	name varchar(128) not null
);

create table recipes (
	id uuid primary key,
	name text not null,
	directions text not null
);

create table recipe_ingredients (
	id uuid primary key,
	recipe_id uuid not null,
	ingredient_id uuid not null,
	quantity real null,
	unit text null,
	required BIT not null
);

create index ingredient_id on recipe_ingredients (ingredient_id);
create index recipe_id on recipe_ingredients (recipe_id);

create table users (
	id uuid primary key,
	name text not null unique,
	password varchar not null,
	email text not null unique,
	permissions SMALLINT not null,
	joined_at timestamptz NOT NULL
);

create table kitchen (
	id uuid primary key,
	user_id uuid not null,
	ingredient_id uuid not null
);

create index kitchen_ingredient_id on kitchen (ingredient_id);
create index user_id on kitchen (user_id);
