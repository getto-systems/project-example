import { env } from "../../../../../../y_environment/env"
import { Authz_pb } from "../../../y_protobuf/db_pb.js"

import { convertRepository } from "../../../../../../../ui/vendor/getto-application/infra/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../../../z_details/_ui/db/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/repository/infra"
import { AuthzRepositoryPod } from "../../infra"
import { FetchDBResult, StoreDBResult } from "../../../../../../z_details/_ui/db/infra"

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
