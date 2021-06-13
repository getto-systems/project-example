import { env } from "../../../../../../y_environment/_ui/env"
import { Authn_pb } from "../../../y_protobuf/db_pb.js"

import { convertRepository } from "../../../../../../../ui/vendor/getto-application/infra/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../../../z_details/_ui/db/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/repository/feature"

import { AuthnRepositoryPod } from "../../infra"
import { FetchDBResult, StoreDBResult } from "../../../../../../z_details/_ui/db/infra"

export function newAuthnRepositoryPod({ webDB }: RepositoryOutsideFeature): AuthnRepositoryPod {
    type Value = Readonly<{
        authAt: string
    }>

    const lastAuth: IndexedDBTarget = {
        store: "authn",
        key: "last",
    }
    const db = initIndexedDB(webDB, {
        database: env.database.authn,
        stores: [lastAuth.store],
    })
    return convertRepository({
        get: (): Promise<FetchDBResult<Value>> => db.get(lastAuth, fromDB),
        set: (value: Value): Promise<StoreDBResult> => db.set(lastAuth, toDB, value),
        remove: (): Promise<StoreDBResult> => db.remove(lastAuth),
    })

    function toDB(value: Value): string {
        return encodeProtobuf(Authn_pb, (message) => {
            message.authAt = value.authAt
        })
    }
    function fromDB(raw: string): Value {
        return decodeProtobuf(Authn_pb, raw)
    }
}
