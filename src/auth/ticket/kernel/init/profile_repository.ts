import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../z_lib/ui/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../ui/vendor/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { AuthProfileRepository, AuthProfileRepositoryValue } from "../infra"
import { authProfileRepositoryConverter } from "../convert"

export function newAuthProfileRepository({ webDB }: RepositoryOutsideFeature): AuthProfileRepository {
    const db = initDB()

    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = authProfileRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(authProfileRepositoryConverter.toRepository(value)),
        remove: () => db.remove(),
    }

    function initDB() {
        const lastAuth: IndexedDBTarget = {
            store: "auth-profile",
            key: "last",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.authProfile,
            stores: [lastAuth.store],
        })

        return {
            get: () => db.get(lastAuth, fromDB),
            set: (value: AuthProfileRepositoryValue) => db.set(lastAuth, toDB, value),
            remove: () => db.remove(lastAuth),
        }

        function toDB(value: AuthProfileRepositoryValue): string {
            return encodeProtobuf(pb.auth.ticket.db.AuthProfile_pb, (message) => {
                message.authAt = value.authAt
                message.roles = value.roles
            })
        }
        function fromDB(raw: string): AuthProfileRepositoryValue {
            return decodeProtobuf(pb.auth.ticket.db.AuthProfile_pb, raw)
        }
    }
}
