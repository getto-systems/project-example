import { LoadMenuDetecter } from "../kernel/method"

import { LoadMenuEvent } from "./event"

export interface LoadMenuPod {
    (detecter: LoadMenuDetecter): LoadMenuMethod
}
export interface LoadMenuMethod {
    <S>(post: Post<LoadMenuEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
