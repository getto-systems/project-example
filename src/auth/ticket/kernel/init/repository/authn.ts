import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../../z_lib/ui/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../z_lib/ui/repository/feature"

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
            return encodeProtobuf(pb.auth.ticket.db.Authn_pb, (message) => {
                message.authAt = value.authAt
            })
        }
        function fromDB(raw: string): AuthnRepositoryValue {
            return decodeProtobuf(pb.auth.ticket.db.Authn_pb, raw)
        }
    }
}
