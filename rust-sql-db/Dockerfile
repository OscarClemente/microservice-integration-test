FROM ubuntu:16.04

# Install Posgresql
RUN apt-key adv --keyserver hkp://p80.pool.sks-keyservers.net:80 --recv-keys B97B0AFCAA1A47F044F244A07FCC7D46ACCC4CF8
RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ precise-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN apt-get update && apt-get install -y python-software-properties software-properties-common postgresql-9.3 postgresql-client-9.3 postgresql-contrib-9.3 libpq-dev sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

USER postgres

RUN    /etc/init.d/postgresql start &&\
    psql --command "CREATE USER docker WITH SUPERUSER PASSWORD 'docker';" &&\
    psql --command "ALTER USER postgres PASSWORD 'password';" &&\
    createdb -O docker docker

RUN echo "host all  all    0.0.0.0/0  md5" >> /etc/postgresql/9.3/main/pg_hba.conf
RUN echo "listen_addresses='*'" >> /etc/postgresql/9.3/main/postgresql.conf
EXPOSE 5432

VOLUME  ["/etc/postgresql", "/var/log/postgresql", "/var/lib/postgresql"]

# Install Rust and our db app
USER docker

RUN echo 'docker' | sudo -S apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
ENV PATH="/home/docker/.cargo/bin:${PATH}"
WORKDIR /usr/src/rust-sql-db
RUN echo 'docker' | sudo -S chown -R docker /usr/src
COPY --chown=docker:docker . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --bin rust-sql-db --path .

ENTRYPOINT ["/bin/bash", "./run.sh"]