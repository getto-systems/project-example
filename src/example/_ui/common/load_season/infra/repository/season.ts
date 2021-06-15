import { env } from "../../../../../../y_environment/_ui/env"
import { Season_pb } from "../../../../y_protobuf/db_pb.js"

import { convertRepository } from "../../../../../../z_details/_ui/repository/helper"
import { FetchDBResult, StoreDBResult } from "../../../../../../z_details/_ui/db/infra"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../../../z_details/_ui/repository/infra/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../../z_details/_ui/repository/feature"

import { SeasonRepositoryPod } from "../../infra"

export function newSeasonRepositoryPod({ webDB }: RepositoryOutsideFeature): SeasonRepositoryPod {
    type Value = Readonly<{
        year: number
    }>

    const currentSeason: IndexedDBTarget = {
        store: "season",
        key: "current",
    }
    const db = initIndexedDB(webDB, {
        database: env.database.season,
        stores: [currentSeason.store],
    })
    return convertRepository({
        get: (): Promise<FetchDBResult<Value>> => db.get(currentSeason, fromDB),
        set: (value: Value): Promise<StoreDBResult> => db.set(currentSeason, toDB, value),
        remove: (): Promise<StoreDBResult> => db.remove(currentSeason),
    })

    function toDB(value: Value): string {
        return encodeProtobuf(Season_pb, (message) => {
            message.year = value.year
        })
    }
    function fromDB(raw: string): Value {
        return decodeProtobuf(Season_pb, raw)
    }
}
