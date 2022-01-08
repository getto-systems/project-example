import { setupActionTestRunner } from "../../../../../ui/vendor/getto-application/action/test_helper"

import { markBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockMultipleBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/init/mock"

import { initSearchColumnsAction, SearchColumnsAction } from "./action"

import { MultipleBoardValueStore } from "../../../../../ui/vendor/getto-application/board/input/infra"
import { initMemoryDB } from "../../repository/init/memory"

describe("SearchColumns", () => {
    test("select columns", async () => {
        const { resource, store } = standard()

        const runner = setupActionTestRunner(resource.field.subscriber)

        await runner(async () => {
            resource.field.load(["column-initial"])
            store.columns.set([markBoardValue("column-a")])
            resource.field.input.publisher.post()
            store.columns.set([markBoardValue("column-a"), markBoardValue("column-b")])
            resource.field.input.publisher.post()
            return resource.field.currentState()
        }).then((stack) => {
            expect(stack).toEqual([
                { type: "succeed-to-load", columns: ["column-initial"] },
                { type: "succeed-to-save", columns: ["column-a"] },
                { type: "succeed-to-save", columns: ["column-a", "column-b"] },
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
            store.columns.set([markBoardValue("column-a")])
            return resource.field.currentState()
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
    resource: Readonly<{ field: SearchColumnsAction }>
    store: Readonly<{
        columns: MultipleBoardValueStore
    }>
}> {
    const resource = {
        field: initSearchColumnsAction({
            columnsRepository: initMemoryDB(),
        }),
    }

    const store = {
        columns: mockMultipleBoardValueStore(),
    }

    resource.field.input.connector.connect(store.columns)

    return { resource, store }
}