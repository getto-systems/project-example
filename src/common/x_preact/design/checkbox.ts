import { PrepareElementState } from "../../../z_lib/ui/prepare/data"
import { CheckboxBoardContent } from "../../../z_vendor/getto-application/board/input/x_preact/checkbox"

export function checkboxOptions<T>(
    state: PrepareElementState<readonly T[]>,
    option: { (value: T): CheckboxBoardContent },
): readonly CheckboxBoardContent[] {
    if (state.isLoad) {
        return state.data.map(option)
    } else {
        return []
    }
}
