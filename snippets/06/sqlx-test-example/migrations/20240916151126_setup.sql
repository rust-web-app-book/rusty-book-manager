-- Add migration script here
create table books (
id serial primary key,
title text not null,
author text not null
);