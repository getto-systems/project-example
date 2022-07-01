import { test, expect } from "vitest"
import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../repository/init/memory"

import { searchColumnsRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchColumnsAction, SearchColumnsAction } from "./action"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

test("select columns", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.field)

    await runner(async () => {
        await resource.field.state.ignitionState
        store.columns.set(["column-a"])
        store.columns.set(["column-a", "column-b"])
        return resource.field.state.currentState()
    }).then((stack) => {
        expect(stack).toEqual([{ type: "success" }, { type: "success" }, { type: "success" }])
        expect(resource.field.get()).toEqual(["column-a", "column-b"])
    })
})

function standard() {
    return initResource()
}

function initResource(): Readonly<{
    resource: Readonly<{ field: SearchColumnsAction }>
    store: Readonly<{
        columns: MultipleBoardValueStore
    }>
}> {
    const resource = {
        field: initSearchColumnsAction({
            columnsRepository: standard_columnRepository(),
        }),
    }

    const store = {
        columns: mockMultipleBoardValueStore(resource.field.input),
    }

    return { resource, store }
}

function standard_columnRepository() {
    const db = initMemoryDB()
    db.set([])
    return convertDB(db, searchColumnsRepositoryConverter)
}
