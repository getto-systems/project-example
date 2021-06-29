import { MenuExpand_pb } from "../../../y_protobuf/db_pb.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../../z_details/_ui/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"
import {
    IndexedDBTarget,
    initIndexedDB,
} from "../../../../../z_details/_ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../../z_details/_ui/repository/feature"

import { MenuExpandRepository, MenuExpandRepositoryValue } from "../../infra"

import { menuExpandRepositoryConverter } from "../../convert"

export type MenuExpandRepositoryParams = Readonly<{
    database: string
    key: string
}>

export function newMenuExpandRepository(
    { webDB }: RepositoryOutsideFeature,
    params: MenuExpandRepositoryParams,
): MenuExpandRepository {
    const db = initDB()
    return {
        get: () =>
            mapFetchRepositoryResult(db.get(), async (value) => {
                const result = menuExpandRepositoryConverter.fromRepository(value)
                if (!result.valid) {
                    return fetchRepositoryRemovedResult(await db.remove())
                }
                return { success: true, found: true, value: result.value }
            }),
        set: (value) => db.set(menuExpandRepositoryConverter.toRepository(value)),
        remove: () => db.remove(),
    }

    function initDB() {
        const menuExpand: IndexedDBTarget = {
            store: "menu-expand",
            key: params.key,
        }
        const db = initIndexedDB(webDB, {
            database: params.database,
            stores: [menuExpand.store],
        })

        return {
            get: () => db.get(menuExpand, fromDB),
            set: (value: MenuExpandRepositoryValue) => db.set(menuExpand, toDB, value),
            remove: () => db.remove(menuExpand),
        }

        function toDB(value: MenuExpandRepositoryValue): string {
            return encodeProtobuf(MenuExpand_pb, (message) => {
                message.paths = value.map((labels) => {
                    const message = new MenuExpand_pb.Path()
                    message.labels = labels
                    return message
                })
            })
        }
        function fromDB(raw: string): MenuExpandRepositoryValue {
            return decodeProtobuf(MenuExpand_pb, raw).paths.map((path) => path.labels || [])
        }
    }
}
