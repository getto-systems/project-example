import { test, expect } from "vitest"
import { observeAtom } from "../../../z_vendor/getto-atom/test_helper"

import { initMemoryDB } from "../repository/detail/memory"

import { searchSidebarRepositoryConverter } from "./convert"
import { convertDB } from "../repository/detail/convert"

import { initToggleSidebarAction, ToggleSidebarAction } from "./action"

test("toggle sidebar", async () => {
    const { sidebar } = standard()

    const result = observeAtom(sidebar.state)

    await sidebar.state.ignitionState
    await sidebar.fold()
    await sidebar.expand()

    expect(result()).toEqual([
        { type: "success", state: { isExpand: true } },
        { type: "success", state: { isExpand: false } },
        { type: "success", state: { isExpand: true } },
    ])
})

function standard(): Readonly<{
    sidebar: ToggleSidebarAction
}> {
    return {
        sidebar: initToggleSidebarAction(
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
