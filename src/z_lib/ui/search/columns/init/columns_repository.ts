import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import { IndexedDBTarget, initIndexedDB } from "../../../repository/init/indexed_db"

import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"
import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../repository/init/helper"

import { RepositoryOutsideFeature } from "../../../repository/feature"

import { searchColumnsRepositoryConverter } from "../convert"

import { SearchColumnsRepository, SearchColumnsRepositoryValue } from "../infra"

export function newSearchColumnsRepository(
    { webDB }: RepositoryOutsideFeature,
    key: string,
): SearchColumnsRepository {
    const db = initDB()
    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = searchColumnsRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(searchColumnsRepositoryConverter.toRepository(value)),
    }

    function initDB() {
        const columns: IndexedDBTarget = {
            store: "search-columns",
            key,
        }
        const db = initIndexedDB(webDB, {
            database: env.database.searchColumns,
            stores: [columns.store],
        })

        return {
            get: () => db.get(columns, fromDB),
            set: (value: SearchColumnsRepositoryValue) => db.set(columns, toDB, value),
            remove: () => db.remove(columns),
        }

        function toDB(value: SearchColumnsRepositoryValue): string {
            return encodeProtobuf(pb.lib.search.db.ColumnsPb, (message) => {
                // value に readonly がついているため、新しく生成する
                message.columns = Array.from(value)
            })
        }
        function fromDB(raw: string): SearchColumnsRepositoryValue {
            const message = decodeProtobuf(pb.lib.search.db.ColumnsPb, raw)
            return message.columns
        }
    }
}
