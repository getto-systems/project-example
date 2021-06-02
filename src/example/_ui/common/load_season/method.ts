import { LoadSeasonEvent } from "./event"

export interface LoadSeasonMethod {
    <S>(post: Post<LoadSeasonEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
