import { ClearAuthTicketEvent } from "./event"

export interface ClearAuthTicketMethod {
    <S>(post: Post<ClearAuthTicketEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
