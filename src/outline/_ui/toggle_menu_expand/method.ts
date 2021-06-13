import { ToggleMenuExpandEvent } from "./event"

import { MenuCategoryPath } from "../kernel/data"
import { LoadMenuDetecter } from "../kernel/method"

export interface ToggleMenuExpandPod {
    (detecter: LoadMenuDetecter): ToggleMenuExpandMethod
}
export interface ToggleMenuExpandMethod {
    <S>(path: MenuCategoryPath, post: Post<ToggleMenuExpandEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
