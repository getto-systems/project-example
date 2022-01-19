import { env } from "../../../y_environment/ui/env"
import pb from "../../../y_protobuf/proto.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../z_lib/ui/repository/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../ui/vendor/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { SeasonRepository, SeasonRepositoryValue } from "../infra"

import { seasonRepositoryConverter } from "../convert"

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
            return encodeProtobuf(pb.example.db.Season_pb, (message) => {
                message.year = value.year
            })
        }
        function fromDB(raw: string): SeasonRepositoryValue {
            return decodeProtobuf(pb.example.db.Season_pb, raw)
        }
    }
}
