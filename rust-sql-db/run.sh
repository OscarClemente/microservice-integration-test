#!/bin/bash
echo 'docker' | sudo -S /etc/init.d/postgresql restart
echo DATABASE_URL=postgres://postgres:password@localhost/docker > .env
diesel migration run
rust-sql-db