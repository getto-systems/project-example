import { MenuExpand_pb } from "../../../y_protobuf/db_pb.js"

import { convertRepository } from "../../../../../../ui/vendor/getto-application/infra/repository/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"
import { IndexedDBTarget, initIndexedDB } from "../../../../../z_details/_ui/db/indexed_db"

import { FetchDBResult, StoreDBResult } from "../../../../../z_details/_ui/db/infra"
import { RepositoryOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/repository/infra"
import { MenuExpandRepositoryPod } from "../../infra"

export type MenuExpandRepositoryParams = Readonly<{
    database: string
    key: string
}>
export function newMenuExpandRepositoryPod(
    { webDB }: RepositoryOutsideFeature,
    params: MenuExpandRepositoryParams,
): MenuExpandRepositoryPod {
    type Value = string[][]

    const menuExpand: IndexedDBTarget = {
        store: "menu-expand",
        key: params.key,
    }
    const db = initIndexedDB(webDB, {
        database: params.database,
        stores: [menuExpand.store],
    })
    return convertRepository({
        get: (): Promise<FetchDBResult<Value>> => db.get(menuExpand, fromDB),
        set: (value: Value): Promise<StoreDBResult> => db.set(menuExpand, toDB, value),
        remove: (): Promise<StoreDBResult> => db.remove(menuExpand),
    })

    function toDB(value: Value): string {
        return encodeProtobuf(MenuExpand_pb, (message) => {
            message.paths = value.map((labels) => {
                const message = new MenuExpand_pb.Path()
                message.labels = labels
                return message
            })
        })
    }
    function fromDB(raw: string): Value {
        return decodeProtobuf(MenuExpand_pb, raw).paths.map((path) => path.labels || [])
    }
}
