import { test, expect } from "vitest"
import {
    FocusModifyListInfra,
    initFocusModifyListAction,
    initFocusRegisterListAction,
} from "./action"
import { LoadState, loadState_loaded } from "../load/data"
import { initAtom } from "../../../z_vendor/getto-atom/atom"
import { DetectFocusListKeyResult } from "./data"

test("focus register-list", () => {
    const arr = [{ name: "name-1" }, { name: "name-2" }, { name: "name-3" }]
    const list = initAtom<LoadState<readonly Data[]>>({
        initialState: loadState_loaded(arr),
    })
    const action = initFocusRegisterListAction<Data>(list.state, (data) => data.name)

    action.focusTo(arr[0])

    expect(action.isEntryFocused(arr[0], action.detect.currentState())).toEqual(true)
    expect(action.isEntryFocused(arr[1], action.detect.currentState())).toEqual(false)

    action.close()

    expect(action.isEntryFocused(arr[0], action.detect.currentState())).toEqual(false)
})

test("focus modify-list", () => {
    const arr = [{ name: "name-1" }, { name: "name-2" }, { name: "name-3" }]
    const list = initAtom<LoadState<readonly Data[]>>({
        initialState: loadState_loaded(arr),
    })
    const action = initFocusModifyListAction<Data>(
        list.state,
        (data) => data.name,
        standard_infra(),
    )

    action.focusTo(arr[0], { y: 0 })

    expect(action.isEntryFocused(arr[0], action.detect.currentState())).toEqual(true)
    expect(action.isEntryFocused(arr[1], action.detect.currentState())).toEqual(false)

    action.close({ y: 0 })

    expect(action.isEntryFocused(arr[0], action.detect.currentState())).toEqual(false)
})

type Data = Readonly<{
    name: string
}>

function standard_infra(): FocusModifyListInfra {
    return {
        detect(): DetectFocusListKeyResult {
            return { found: false }
        },
        update(_state: DetectFocusListKeyResult): void {
            return
        },
    }
}
