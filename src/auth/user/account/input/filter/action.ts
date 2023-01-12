import {
    initMultipleFilterAction,
    MultipleFilterAction,
} from "../../../../../common/util/input/filter/multiple"

import { AuthPermission } from "../../../kernel/data"

export type AuthPermissionGrantedFilterAction = MultipleFilterAction

export function initAuthPermissionGrantedFilterAction(
    initial: readonly AuthPermission[],
): Readonly<{
    input: AuthPermissionGrantedFilterAction
    setOptions: { (state: readonly AuthPermission[]): void }
    pin: () => readonly AuthPermission[]
}> {
    return initMultipleFilterAction({
        initial,
        convert: (data) => data,
    })
}
