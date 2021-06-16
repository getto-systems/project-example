import { env } from "../../../../../../y_environment/_ui/env"
import { Season_pb } from "../../../../y_protobuf/db_pb.js"

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

import { SeasonRepository, SeasonRepositoryValue } from "../../infra"

import { seasonRepositoryConverter } from "../../converter"

export function newSeasonRepository({ webDB }: RepositoryOutsideFeature): SeasonRepository {
    const db = initDB()
    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = seasonRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(seasonRepositoryConverter.toRepository(value)),
        remove: () => db.remove(),
    }

    function initDB() {
        const currentSeason: IndexedDBTarget = {
            store: "season",
            key: "current",
        }
        const db = initIndexedDB(webDB, {
            database: env.database.season,
            stores: [currentSeason.store],
        })
        return {
            get: () => db.get(currentSeason, fromDB),
            set: (value: SeasonRepositoryValue) => db.set(currentSeason, toDB, value),
            remove: () => db.remove(currentSeason),
        }

        function toDB(value: SeasonRepositoryValue): string {
            return encodeProtobuf(Season_pb, (message) => {
                message.year = value.year
            })
        }
        function fromDB(raw: string): SeasonRepositoryValue {
            return decodeProtobuf(Season_pb, raw)
        }
    }
}
