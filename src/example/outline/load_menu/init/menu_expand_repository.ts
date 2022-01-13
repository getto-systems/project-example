import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchRepositoryRemovedResult,
    mapFetchRepositoryResult,
} from "../../../../z_lib/ui/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../ui/vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../z_lib/ui/repository/init/indexed_db"

import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

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
            return encodeProtobuf(pb.example.outline.db.MenuExpand_pb, (message) => {
                message.paths = value.map((labels) => {
                    const message = new pb.example.outline.db.MenuExpand_pb.Path()
                    message.labels = Array.from(labels)
                    return message
                })
            })
        }
        function fromDB(raw: string): MenuExpandRepositoryValue {
            return decodeProtobuf(pb.example.outline.db.MenuExpand_pb, raw).paths.map(
                (path) => path.labels || [],
            )
        }
    }
}
