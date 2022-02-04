import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../z_vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../infra"
import { authTicketRepositoryConverter } from "../convert"
import { convertDB } from "../../../../z_lib/ui/repository/init/convert"

export function newAuthTicketRepository({ webDB }: RepositoryOutsideFeature): AuthTicketRepository {
    return convertDB(initDB(), authTicketRepositoryConverter)

    function initDB() {
        const target: IndexedDBTarget = {
            store: "auth-profile",
            key: "last",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.authProfile,
            stores: [target.store],
        })

        return {
            get: () => db.get(target, fromDB),
            set: (value: AuthTicketRepositoryValue) => db.set(target, toDB, value),
            remove: () => db.remove(target),
        }

        function toDB(value: AuthTicketRepositoryValue): string {
            return encodeProtobuf(pb.auth.ticket.db.AuthProfile_pb, (message) => {
                message.authAt = value.authAt
                message.roles = Array.from(value.roles)
            })
        }
        function fromDB(raw: string): AuthTicketRepositoryValue {
            return decodeProtobuf(pb.auth.ticket.db.AuthProfile_pb, raw)
        }
    }
}
