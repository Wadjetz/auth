# Bauth

An OAuth server implementation (Work in progress)

## Usage with docker

```yml
version: '3.7'
services:
    bauth_server:
        image: wadjetz/bauth
        container_name: bauth_server
        environment:
            DATABASE_URL: postgres://bauth:bauth@bauth_database/bauth
        depends_on:
            - bauth_database

    bauth_database:
        image: postgres:12.2-alpine
        container_name: bauth_database
        environment:
            POSTGRES_PASSWORD: bauth
            POSTGRES_USER: bauth
            POSTGRES_DB: bauth
        volumes:
            - .docker_data:/var/lib/postgresql/data
```
