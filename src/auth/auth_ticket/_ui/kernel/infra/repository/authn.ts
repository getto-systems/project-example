import { env } from "../../../../../../y_environment/_ui/env"
import { Authn_pb } from "../../../y_protobuf/db_pb.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../../../z_details/_ui/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../../../z_details/_ui/repository/infra/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../../z_details/_ui/repository/feature"

import { AuthnRepository, AuthnRepositoryValue } from "../../infra"
import { authnRepositoryConverter } from "../../convert"

export function newAuthnRepository({ webDB }: RepositoryOutsideFeature): AuthnRepository {
    const db = initDB()

    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = authnRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(authnRepositoryConverter.toRepository(value)),
        remove: () => db.remove(),
    }

    function initDB() {
        const lastAuth: IndexedDBTarget = {
            store: "authn",
            key: "last",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.authn,
            stores: [lastAuth.store],
        })

        return {
            get: () => db.get(lastAuth, fromDB),
            set: (value: AuthnRepositoryValue) => db.set(lastAuth, toDB, value),
            remove: () => db.remove(lastAuth),
        }

        function toDB(value: AuthnRepositoryValue): string {
            return encodeProtobuf(Authn_pb, (message) => {
                message.authAt = value.authAt
            })
        }
        function fromDB(raw: string): AuthnRepositoryValue {
            return decodeProtobuf(Authn_pb, raw)
        }
    }
}
