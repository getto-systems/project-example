import { LoadMenuLocationDetecter } from "../kernel/method";
import { UpdateMenuBadgeEvent } from "./event"

export interface UpdateMenuBadgePod {
    (detecter: LoadMenuLocationDetecter): UpdateMenuBadgeMethod
}
export interface UpdateMenuBadgeMethod {
    <S>(post: Post<UpdateMenuBadgeEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
