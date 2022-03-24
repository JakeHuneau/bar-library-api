create table ingredients
(
	id SERIAL primary key,
	name varchar(128) not null
);

create table recipes
(
	id SERIAL primary key,
	name text not null,
	directions text not null
);

create table recipe_ingredients
(
	id SERIAL primary key,
	recipe_id int not null,
	ingredient_id int not null,
	quantity real null,
	unit text null,
	required BIT not null
);

create index ingredient_id
	on recipe_ingredients (ingredient_id);

create index recipe_id
	on recipe_ingredients (recipe_id);

create table users
(
	id SERIAL primary key,
	name varchar(256) not null,
	password varchar not null,
	permissions SMALLINT not null
);

create table kitchen
(
	id SERIAL primary key,
	user_id int not null,
	ingredient_id int not null
);

create index kitchen_ingredient_id
	on kitchen (ingredient_id);

create index user_id
	on kitchen (user_id);

