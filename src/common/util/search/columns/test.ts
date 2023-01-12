import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"

import { mockMultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/test_helper"
import { initMemoryDB } from "../../repository/init/memory"

import { searchColumnsRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchColumnsAction, SearchColumnsAction, visibleKeys } from "./action"

import { MultipleBoardValueStore } from "../../../../z_vendor/getto-application/board/input/infra"
import { ticker } from "../../timer/helper"

test("select columns", async () => {
    const { field, store } = standard()

    expect(
        await observeApplicationState(field.state, async () => {
            await field.state.ignitionState

            store.columns.set(["column-a"])
            await ticker({ wait_millisecond: 0 }, () => null)

            store.columns.set(["column-a", "column-b"])
            return field.state.currentState()
        }),
    ).toEqual([
        { type: "columns", visibleKeys: ["stored"] },
        { type: "columns", visibleKeys: ["column-a"] },
        { type: "columns", visibleKeys: ["column-a", "column-b"] },
    ])
})

test("visibleKeys", async () => {
    expect(visibleKeys({ type: "columns", visibleKeys: ["key"] })).toEqual(["key"])
})

function standard(): Readonly<{
    field: SearchColumnsAction
    store: Readonly<{
        columns: MultipleBoardValueStore
    }>
}> {
    const field = initSearchColumnsAction(
        {
            columnsRepository: standard_columnRepository(),
        },
        ["initial"],
    )

    return {
        field,
        store: {
            columns: mockMultipleBoardValueStore(field.input),
        },
    }
}

function standard_columnRepository() {
    const db = initMemoryDB()
    db.set(["stored"])
    return convertDB(db, searchColumnsRepositoryConverter)
}
