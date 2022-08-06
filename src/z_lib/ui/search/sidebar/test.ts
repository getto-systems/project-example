import { test, expect } from "vitest"
import { observeApplicationState } from "../../../../z_vendor/getto-application/action/test_helper"

import { initMemoryDB } from "../../repository/init/memory"

import { searchSidebarRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchSidebarAction, SearchSidebarAction } from "./action"

test("select columns", async () => {
    const { sidebar } = standard()

    expect(
        await observeApplicationState(sidebar.state, async () => {
            await sidebar.state.ignitionState
            await sidebar.fold()
            await sidebar.expand()
            return sidebar.state.currentState()
        }),
    ).toEqual([
        { type: "success", state: { isExpand: true } },
        { type: "success", state: { isExpand: false } },
        { type: "success", state: { isExpand: true } },
    ])
})

function standard(): Readonly<{
    sidebar: SearchSidebarAction
}> {
    return {
        sidebar: initSearchSidebarAction(
            {
                sidebarRepository: standard_sidebarRepository(),
            },
            { isExpand: true },
        ),
    }
}

function standard_sidebarRepository() {
    const db = initMemoryDB()
    db.set({ isExpand: true })
    return convertDB(db, searchSidebarRepositoryConverter)
}
