import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../repository/init/memory"

import { searchColumnsRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchColumnsAction, SearchColumnsAction } from "./action"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

test("select columns", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner(resource.field.subscriber)

    await runner(async () => {
        await resource.field.ignitionState
        await resource.field.set(["column-initial"])
        store.columns.set(["column-a"])
        store.columns.set(["column-a", "column-b"])
        return resource.field.currentState()
    }).then((stack) => {
        expect(stack).toEqual([
            { type: "success", columns: [] },
            { type: "success", columns: ["column-initial"] },
            { type: "success", columns: ["column-a"] },
            { type: "success", columns: ["column-a", "column-b"] },
        ])
    })
})

test("terminate", async () => {
    const { resource, store } = standard()

    const runner = setupActionTestRunner({
        subscribe: (handler) => {
            resource.field.subscriber.subscribe(handler)
        },
        unsubscribe: () => null,
    })

    await runner(async () => {
        resource.field.terminate()
        store.columns.set(["column-a"])
        return resource.field.currentState()
    }).then((stack) => {
        // no input/validate event after terminate
        expect(stack).toEqual([])
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
