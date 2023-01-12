import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../z_vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../common/util/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { SeasonRepository, SeasonRepositoryValue } from "../infra"

import { seasonRepositoryConverter } from "../convert"
import { convertDB } from "../../../../common/util/repository/init/convert"

import { Season } from "../data"

export function newSeasonRepository(
    { webDB }: RepositoryOutsideFeature,
    availableSeasons: readonly Season[],
): SeasonRepository {
    return convertDB(initDB(), seasonRepositoryConverter(availableSeasons))

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
            return encodeProtobuf(pb.common.season.db.SeasonPb, (message) => {
                message.year = value.season.year
                message.period = value.season.period
                message.expires = value.expires
            })
        }
        function fromDB(raw: string): SeasonRepositoryValue {
            const value = decodeProtobuf(pb.common.season.db.SeasonPb, raw)
            return {
                season: { year: value.year, period: value.period },
                expires: value.expires,
            }
        }
    }
}
