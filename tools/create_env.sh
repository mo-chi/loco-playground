#!/bin/bash

create_env_file () {
    ENV_FILE_PATH=$1
    ENV=$2

    if [[ -e ${ENV_FILE_PATH} ]]; then
        . ${ENV_FILE_PATH}
    fi

    POSTGRES_DB=${POSTGRES_DB:-"loco_playground_${ENV}"}
    POSTGRES_USER=${POSTGRES_USER:-"loco"}
    POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-"loco"}

    cat << EOF > ${ENV_FILE_PATH}
POSTGRES_DB=${POSTGRES_DB}
POSTGRES_USER=${POSTGRES_USER}
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
DATABASE_URL=postgres://\${POSTGRES_USER}:\${POSTGRES_PASSWORD}@postgres:5432/\${POSTGRES_DB}
EOF

    cat ${ENV_FILE_PATH}
}

case "$1" in
    "test")
        echo "create_env_file ./.env.test"
        create_env_file "./.env.test" $1
        ;;
    "development")
        echo "create_env_file ./.env"
        create_env_file "./.env" $1
        ;;
    *)
        echo "Usage: bash tools/create_env.sh [test | development]"
        ;;
esac
