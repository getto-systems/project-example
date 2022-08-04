import { PrepareElementState } from "../../../z_lib/ui/prepare/data"
import { SelectBoardContent } from "../../../z_vendor/getto-application/board/input/x_preact/select"

export function selectOptions<T>(
    state: PrepareElementState<readonly T[]>,
    option: { (value: T): SelectBoardContent },
): readonly SelectBoardContent[] {
    if (state.isLoad) {
        return state.data.map(option)
    } else {
        return [loading()]
    }
}

export function selectOptionsWithAll<T>(
    state: PrepareElementState<readonly T[]>,
    option: { (value: T): SelectBoardContent },
): readonly SelectBoardContent[] {
    if (state.isLoad) {
        return [all(), ...state.data.map(option)]
    } else {
        return [loading()]
    }
}

export function selectOptionsWithPrompt<T>(
    state: PrepareElementState<readonly T[]>,
    option: { (value: T): SelectBoardContent },
): readonly SelectBoardContent[] {
    if (state.isLoad) {
        return [prompt(), ...state.data.map(option)]
    } else {
        return [loading()]
    }
}

function loading(): SelectBoardContent {
    return { key: "", value: "", label: "読み込み中" }
}

function all(): SelectBoardContent {
    return { key: "", value: "", label: "すべて" }
}

function prompt(): SelectBoardContent {
    return { key: "", value: "", label: "選択してください" }
}
