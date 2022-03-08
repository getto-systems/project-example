import { setupActionTestRunner } from "../../../../z_vendor/getto-application/action/test_helper"

import { initMemoryDB } from "../../repository/init/memory"

import { searchSidebarRepositoryConverter } from "./convert"
import { convertDB } from "../../repository/init/convert"

import { initSearchSidebarAction, SearchSidebarAction } from "./action"

describe("SearchSidebar", () => {
    test("select columns", async () => {
        const { sidebar } = standard()

        const runner = setupActionTestRunner(sidebar.subscriber)

        await runner(async () => {
            await sidebar.ignitionState
            await sidebar.fold()
            await sidebar.expand()
            return sidebar.currentState()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "succeed-to-load", state: { isExpand: true } },
                { type: "succeed-to-save", state: { isExpand: false } },
                { type: "succeed-to-save", state: { isExpand: true } },
            ])
        })
    })

    test("terminate", async () => {
        const { sidebar } = standard()

        const runner = setupActionTestRunner(sidebar.subscriber)

        await runner(async () => {
            sidebar.terminate()
            sidebar.fold()
            return sidebar.currentState()
        }).then((stack) => {
            // no input/validate event after terminate
            expect(stack).toEqual([])
        })
    })
})

function standard() {
    return initResource()
}

function initResource(): Readonly<{
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
