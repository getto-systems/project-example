import {
    initMultipleFilterAction,
    MultipleFilterAction,
} from "../../../../../z_lib/ui/input/filter/multiple"

import { AuthRole } from "../../../kernel/data"

export type AuthUserGrantedRolesFilterAction = MultipleFilterAction

export function initAuthUserGrantedRolesFilterAction(initial: readonly AuthRole[]): Readonly<{
    input: AuthUserGrantedRolesFilterAction
    setOptions: { (state: readonly AuthRole[]): void }
    pin: () => readonly AuthRole[]
}> {
    return initMultipleFilterAction({
        initial,
        convert: (data) => data,
    })
}
