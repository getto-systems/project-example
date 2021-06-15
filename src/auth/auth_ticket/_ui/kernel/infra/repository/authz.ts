import { env } from "../../../../../../y_environment/_ui/env"
import { Authz_pb } from "../../../y_protobuf/db_pb.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import {
    convertRepository,
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../../../z_details/_ui/repository/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../../../z_details/_ui/repository/infra/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../../z_details/_ui/repository/feature"

import { AuthzRepository, AuthzRepositoryPod, AuthzRepositoryValue } from "../../infra"
import { FetchDBResult, StoreDBResult } from "../../../../../../z_details/_ui/db/infra"

import { authzRepositoryConverter } from "../../converter"

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

export function newAuthzRepositoryPod({ webDB }: RepositoryOutsideFeature): AuthzRepositoryPod {
    type Value = Readonly<{
        roles: string[]
    }>

    const lastAuth: IndexedDBTarget = {
        store: "authz",
        key: "last",
    }
    const db = initIndexedDB(webDB, {
        database: env.database.authz,
        stores: [lastAuth.store],
    })
    return convertRepository({
        get: (): Promise<FetchDBResult<Value>> => db.get(lastAuth, fromDB),
        set: (value: Value): Promise<StoreDBResult> => db.set(lastAuth, toDB, value),
        remove: (): Promise<StoreDBResult> => db.remove(lastAuth),
    })
    function toDB(value: Value): string {
        return encodeProtobuf(Authz_pb, (message) => {
            message.roles = value.roles
        })
    }
    function fromDB(raw: string): Value {
        return decodeProtobuf(Authz_pb, raw)
    }
}
