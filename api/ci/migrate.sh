#!/bin/sh

migrate_main() {
    local cloudsql_proxy
    cloudsql_proxy=$1

    if [ ! -x "${cloudsql_proxy}" ]; then
        echo "usage: ./migrate.sh <path-to-cloudsql_proxy>"
        exit 1
    fi

    if [ -z "${SQL_INSTANCE}" ]; then
        echo "SQL_INSTANCE is not specified"
        exit 1
    fi
    if [ -z "${SQL_CREDENTIAL}" ]; then
        echo "SQL_CREDENTIAL is not specified"
        exit 1
    fi

    #$cloudsql_proxy -instances=${SQL_INSTANCE}=tcp:0.0.0.0:3306 -credential_file=${SQL_CREDENTIAL} &

    #npx prisma migrate deploy --schema=src/auth/x_prisma/schema.prisma
}

migrate_main $@
