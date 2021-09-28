import { env } from "../../../../../y_environment/_ui/env"
import { Authz_pb } from "../../../../../y_protobuf/proto.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"
import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../../z_lib/ui/repository/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"

import { AuthzRepository, AuthzRepositoryValue } from "../../infra"

import { authzRepositoryConverter } from "../../convert"

export function newAuthzRepository({ webDB }: RepositoryOutsideFeature): AuthzRepository {
    const db = initDB()

    return {
        get: async () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = authzRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(authzRepositoryConverter.toRepository(value)),
        remove: () => db.remove(),
    }

    function initDB() {
        const lastAuth: IndexedDBTarget = {
            store: "authz",
            key: "last",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.authz,
            stores: [lastAuth.store],
        })

        return {
            get: () => db.get(lastAuth, fromDB),
            set: (value: AuthzRepositoryValue) => db.set(lastAuth, toDB, value),
            remove: () => db.remove(lastAuth),
        }

        function toDB(value: AuthzRepositoryValue): string {
            return encodeProtobuf(Authz_pb, (message) => {
                message.roles = value.roles
            })
        }
        function fromDB(raw: string): AuthzRepositoryValue {
            return decodeProtobuf(Authz_pb, raw)
        }
    }
}
