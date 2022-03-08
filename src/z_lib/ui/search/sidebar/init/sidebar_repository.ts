import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import { IndexedDBTarget, initIndexedDB } from "../../../repository/init/indexed_db"

import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"
import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../repository/init/helper"

import { RepositoryOutsideFeature } from "../../../repository/feature"

import { searchSidebarRepositoryConverter } from "../convert"

import { SearchSidebarRepository } from "../infra"

import { SearchSidebarExpand } from "../data"

export function newSearchSidebarRepository(
    { webDB }: RepositoryOutsideFeature,
    key: string,
): SearchSidebarRepository {
    const db = initDB()
    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = searchSidebarRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(searchSidebarRepositoryConverter.toRepository(value)),
    }

    function initDB() {
        const target: IndexedDBTarget = {
            store: "search-sidebar",
            key,
        }
        const db = initIndexedDB(webDB, {
            database: env.database.sidebar,
            stores: [target.store],
        })

        return {
            get: () => db.get(target, fromDB),
            set: (value: SearchSidebarExpand) => db.set(target, toDB, value),
            remove: () => db.remove(target),
        }

        function toDB(value: SearchSidebarExpand): string {
            return encodeProtobuf(pb.lib.search.sidebar.db.SidebarPb, (message) => {
                // value に readonly がついているため、新しく生成する
                message.isExpand = value.isExpand
            })
        }
        function fromDB(raw: string): SearchSidebarExpand {
            const message = decodeProtobuf(pb.lib.search.sidebar.db.SidebarPb, raw)
            return { isExpand: message.isExpand }
        }
    }
}
