import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../util/repository/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../z_vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../util/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../util/repository/feature"

import { MenuExpandRepository, MenuExpandRepositoryValue } from "../infra"

import { menuExpandRepositoryConverter } from "../convert"

export type MenuExpandRepositoryParams = Readonly<{
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
            database: env.database.menuExpand,
            stores: [menuExpand.store],
        })

        return {
            get: () => db.get(menuExpand, fromDB),
            set: (value: MenuExpandRepositoryValue) => db.set(menuExpand, toDB, value),
            remove: () => db.remove(menuExpand),
        }

        function toDB(value: MenuExpandRepositoryValue): string {
            return encodeProtobuf(pb.common.outline.db.MenuExpandPb, (message) => {
                message.paths = value.map((labels) => {
                    const message = new pb.common.outline.db.MenuExpandPb.Path()
                    message.labels = Array.from(labels)
                    return message
                })
            })
        }
        function fromDB(raw: string): MenuExpandRepositoryValue {
            return decodeProtobuf(pb.common.outline.db.MenuExpandPb, raw).paths.map(
                (path) => path.labels || [],
            )
        }
    }
}
