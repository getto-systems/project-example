import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../common/util/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../common/util/repository/detail/indexed_db"

import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { AuthTicketRepository, AuthTicketRepositoryValue } from "../infra"
import { authTicketRepositoryConverter } from "../convert"
import { convertDB } from "../../../../common/util/repository/detail/convert"

export function newAuthTicketRepository({ webDB }: RepositoryOutsideFeature): AuthTicketRepository {
    return convertDB(initDB(), authTicketRepositoryConverter)

    function initDB() {
        const target: IndexedDBTarget = {
            store: "auth-ticket",
            key: "last",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.authTicket,
            stores: [target.store],
        })

        return {
            get: () => db.get(target, fromDB),
            set: (value: AuthTicketRepositoryValue) => db.set(target, toDB, value),
            remove: () => db.remove(target),
        }

        function toDB(value: AuthTicketRepositoryValue): string {
            return encodeProtobuf(pb.auth.ticket.db.AuthTicketPb, (message) => {
                message.authAt = value.authAt
                message.granted = Array.from(value.granted)
            })
        }
        function fromDB(raw: string): AuthTicketRepositoryValue {
            return decodeProtobuf(pb.auth.ticket.db.AuthTicketPb, raw)
        }
    }
}
