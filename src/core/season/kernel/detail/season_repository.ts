import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import { decodeProtobuf, encodeProtobuf } from "../../../../common/util/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../common/util/repository/detail/indexed_db"

import { RepositoryOutsideFeature } from "../../../../common/util/repository/feature"

import { allSeasons } from "./all_seasons"
import { currentSeason } from "./current_season"
import { seasonRepositoryConverter } from "../convert"
import { convertDB } from "../../../../common/util/repository/detail/convert"

import { Clock } from "../../../../common/util/clock/infra"
import { SeasonRepository, SeasonRepositoryValue } from "../infra"

import { Season } from "../data"

export function newSeasonRepository(
    { webDB }: RepositoryOutsideFeature,
    clock: Clock,
): [SeasonRepository, readonly Season[]] {
    const defaultSeason = currentSeason(clock)
    const availableSeasons = allSeasons(defaultSeason)

    return [convertDB(initDB(), seasonRepositoryConverter(availableSeasons)), availableSeasons]

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
