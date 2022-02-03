import { markBoardValue } from "../../../../ui/vendor/getto-application/board/kernel/mock"
import { mockBoardValueStore } from "../../../../ui/vendor/getto-application/board/input/init/mock"

import { initInputSeasonAction } from "./action"

describe("InputSeason", () => {
    test("get value", async () => {
        const { store, get } = standard()

        store.set(markBoardValue("2021.summer"))
        expect(get()).toEqual("2021.summer")
    })

    test("set value", async () => {
        const { store, set } = standard()

        set(markBoardValue("2021.summer"))
        expect(store.get()).toEqual("2021.summer")
    })

    test("terminate", async () => {
        const { action } = standard()

        action.terminate()

        // input action では特に subscribe しないので確認することがない
        expect(true).toBe(true)
    })
})

function standard() {
    const { input: action, get, set } = initInputSeasonAction()
    const store = mockBoardValueStore()
    action.input.connector.connect(store)

    return { action, store, get, set }
}
