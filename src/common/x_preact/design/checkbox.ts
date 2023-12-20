import { CheckboxBoardContent } from "../../util/board/input/x_preact/checkbox"

import { LoadState } from "../../util/load/data"

export function checkboxOptions<T>(
    state: LoadState<readonly T[]>,
    option: { (value: T): CheckboxBoardContent },
): readonly CheckboxBoardContent[] {
    if (state.isLoad) {
        return state.data.map(option)
    } else {
        return []
    }
}
