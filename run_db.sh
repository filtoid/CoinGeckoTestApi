docker run -ti \
    --env POSTGRES_USER=postgres \
    --env POSTGRES_PASSWORD=postgres \
    --env POSTGRES_HOST_AUTH_METHOD=trust \
    --env PGDATA=/tmp \
    -p '5438:5432' \
	-v $PWD/data:/var/lib/postgresql/data:rw \
    --mount type=bind,src=$PWD/sql/create_database.sql,dst=/docker-entrypoint-initdb.d/create_tables.sql \
    postgres:14.1-alpine