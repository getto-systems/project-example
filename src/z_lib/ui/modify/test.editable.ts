import { test, expect } from "vitest"

import { observeApplicationState } from "../../../z_vendor/getto-application/action/test_helper"

import { EditableDataProps, initEditableDataHandler } from "./action"

test("focus / update / close", async () => {
    const { editable, data, handler } = standard()

    const item: Data = { id: 1, name: "name" }
    const updatedItem: Data = { id: 1, name: "updated-name" }

    expect(
        await observeApplicationState(editable.state, async () => {
            await editable.state.ignitionState

            expect(data()).toEqual({ isLoad: false })

            handler.focus(item)
            expect(data()).toEqual({ isLoad: true, data: item })

            editable.open()
            handler.update(updatedItem)
            expect(data()).toEqual({ isLoad: true, data: updatedItem })

            handler.close()
            expect(data()).toEqual({ isLoad: false })

            return editable.state.currentState()
        }),
    ).toEqual([{ isEditable: false }, { isEditable: true }, { isEditable: false }])
})

function standard(): EditableDataProps<Data> {
    return initEditableDataHandler()
}

type Data = Readonly<{ id: number; name: string }>
