import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../repository/init/memory"

import { searchColumnsRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchColumnsAction, SearchColumnsAction } from "./action"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"

test("select columns", async () => {
    const { field, store } = standard()

    expect(
        await observeApplicationState(field.state, async () => {
            await field.state.ignitionState
            store.columns.set(["column-a"])
            store.columns.set(["column-a", "column-b"])
            return field.state.currentState()
        }),
    ).toEqual([{ type: "success" }, { type: "success" }, { type: "success" }])

    expect(field.get()).toEqual(["column-a", "column-b"])
})

function standard(): Readonly<{
    field: SearchColumnsAction
    store: Readonly<{
        columns: MultipleBoardValueStore
    }>
}> {
    const field = initSearchColumnsAction({
        columnsRepository: standard_columnRepository(),
    })

    return {
        field,
        store: {
            columns: mockMultipleBoardValueStore(field.input),
        },
    }
}

function standard_columnRepository() {
    const db = initMemoryDB()
    db.set([])
    return convertDB(db, searchColumnsRepositoryConverter)
}
