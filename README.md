# Astro Locker

## Description

A simple web application written in Rust for bart Massey and Casey Bailey's CS-510 Rust Web Development Course at PSU in Summer 2023.

## What Was Built

## What Worked

## What Didn't Work

- Registration failures don't gracefully redirect.
- I have spent a lot of time fighting with strings and the concept of borrowed values. Often took the easy way out with .clone() but I'm sure that's improper.
- I've also made some functionality that require DB calls when it probably wasn't necessary. I'm thinking specifically with admins having to make 2 db calls to promote / demote admins. I'm thinking this is more a SQL design problem (I wanted to try joining tables)
- When going to the frontend, a lot of my database functions needed to be drastically changed to accomodate forms instead of straight queries
- I was pretty strapped for time so I never got to making any tests.

## What Was Learned
