import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import { IndexedDBTarget, initIndexedDB } from "../../repository/detail/indexed_db"

import { decodeProtobuf, encodeProtobuf } from "../../protobuf/helper"
import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../repository/detail/helper"

import { RepositoryOutsideFeature } from "../../repository/feature"

import { searchSidebarRepositoryConverter } from "../convert"

import { ToggleSidebarRepository } from "../infra"

import { SidebarExpand } from "../data"

export function newSearchSidebarRepository(
    { webDB }: RepositoryOutsideFeature,
    key: string,
): ToggleSidebarRepository {
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
            set: (value: SidebarExpand) => db.set(target, toDB, value),
            remove: () => db.remove(target),
        }

        function toDB(value: SidebarExpand): string {
            return encodeProtobuf(pb.z_lib.sidebar.db.SidebarPb, (message) => {
                // value に readonly がついているため、新しく生成する
                message.isExpand = value.isExpand
            })
        }
        function fromDB(raw: string): SidebarExpand {
            const message = decodeProtobuf(pb.z_lib.sidebar.db.SidebarPb, raw)
            return { isExpand: message.isExpand }
        }
    }
}
