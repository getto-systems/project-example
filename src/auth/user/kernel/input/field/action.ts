import { Atom } from "../../../../../z_vendor/getto-atom/atom"
import { LoadState } from "../../../../../common/util/load/data"
import {
    MultipleFieldBoard,
    initMultipleFieldBoard,
} from "../../../../../common/util/board/field/action"
import { BoardInitializer } from "../../../../../common/util/board/input/action"

import { AuthPermission } from "../../data"

export type AuthPermissionGrantedField = MultipleFieldBoard<AuthPermission>

export function initAuthPermissionGrantedField(
    options: Atom<LoadState<readonly AuthPermission[]>>,
): [AuthPermissionGrantedField, BoardInitializer<readonly AuthPermission[]>] {
    return initMultipleFieldBoard(options, {
        convert: (data) => data,
    })
}
