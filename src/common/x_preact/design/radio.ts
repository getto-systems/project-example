import { RadioBoardContent } from "../../util/board/input/x_preact/radio"

import { LoadState } from "../../util/load/data"

export function radioOptions<T>(
    state: LoadState<readonly T[]>,
    option: (value: T) => RadioBoardContent,
): readonly RadioBoardContent[] {
    if (state.isLoad) {
        return state.data.map(option)
    } else {
        return [loading()]
    }
}

function loading(): RadioBoardContent {
    return { key: "", value: "", label: "読み込み中" }
}
