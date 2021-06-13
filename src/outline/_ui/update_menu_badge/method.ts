import { LoadMenuDetecter } from "../kernel/method";
import { UpdateMenuBadgeEvent } from "./event"

export interface UpdateMenuBadgePod {
    (detecter: LoadMenuDetecter): UpdateMenuBadgeMethod
}
export interface UpdateMenuBadgeMethod {
    <S>(post: Post<UpdateMenuBadgeEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
