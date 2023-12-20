import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { ticker } from "../../timer/helper"

import { mockMultipleBoardStore } from "../../board/input/test_helper"
import { initMemoryDB } from "../../repository/detail/memory"

import { searchColumnsRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/detail/convert"

import { initSearchColumnsBoard, SearchColumnsBoard } from "./action"

import { MultipleBoardStore } from "../../board/input/infra"

test("select columns", async () => {
    const { field, store } = standard()

    const result = observeAtom(field.filter)

    await field.state.ignitionState

    store.columns.set(["column-a", "column-b"])
    await ticker({ wait_millisecond: 0 }, () => null)

    store.columns.set(["column-a"])
    await ticker({ wait_millisecond: 0 }, () => null)

    expect(result()).toEqual([["column-a"], ["column-a", "column-b"], ["column-a"]])
})

function standard(): Readonly<{
    field: SearchColumnsBoard
    store: Readonly<{
        columns: MultipleBoardStore
    }>
}> {
    const field = initSearchColumnsBoard(
        {
            columnsRepository: standard_columnRepository(),
        },
        [
            {
                key: "column-a",
                content: "A",
                isInitiallyVisible: true,
            },
            {
                key: "column-b",
                content: "B",
                isInitiallyVisible: true,
            },
        ],
    )

    return {
        field,
        store: {
            columns: mockMultipleBoardStore(field.input),
        },
    }
}

function standard_columnRepository() {
    const db = initMemoryDB()
    db.set(["column-a"])
    return convertDB(db, searchColumnsRepositoryConverter)
}
