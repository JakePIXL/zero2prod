# Zero to Production in Rust
[![Quality & Coverage](https://github.com/JakePIXL/zero2prod/actions/workflows/general.yml/badge.svg)](https://github.com/JakePIXL/zero2prod/actions/workflows/general.yml)
[![Security audit](https://github.com/JakePIXL/zero2prod/actions/workflows/audit-on-push.yml/badge.svg)](https://github.com/JakePIXL/zero2prod/actions/workflows/audit-on-push.yml)
[![Fly Deployment](https://github.com/JakePIXL/zero2prod/actions/workflows/deploy.yml/badge.svg)](https://github.com/JakePIXL/zero2prod/actions/workflows/deploy.yml)

This repository contains the code and notes from my learning journey through the book ["Zero to Production in Rust: an opinionated introduction to backend development" by Luca Palmieri.](https://www.zero2prod.com/)

I hope that by sharing my code and notes, it may be helpful to others who are also learning Rust and backend development.


This book is easily ⭐️⭐️⭐️⭐️'s by the way, Luca and contributers if you see this, thank you for your work!


## What I do differently

There are a few things I do differently to the book, which I will list here:

### [Fly.io](https://fly.io/) instead of Digital Ocean
- It's a lot cheaper than DigitalOcean
- It's a lot easier to set up than DigitalOcean (in my opinion)
- It's a lot easier to deploy to than DigitalOcean (in my opinion)

some difficulties you have to overcome are that when fly creates a database it only links the DATABASE_URL to the database, not the username and password. So you have to manually add those to the secrets, or write the code grab the DATABASE_URL and parse it to get the username and password or return the DATABASE_URL itself.