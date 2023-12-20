import { test, expect } from "vitest"
import { observeAtom } from "../../../../z_vendor/getto-atom/test_helper"
import { initEditableBoardAction } from "./action"

test("open / close", async () => {
    const { action } = standard()

    const result = observeAtom(action.state)

    action.open()
    action.close()

    expect(result()).toEqual([{ isEditable: true }, { isEditable: false }])
})

function standard() {
    return { action: initEditableBoardAction() }
}
