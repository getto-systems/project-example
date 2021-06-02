import { SaveAuthTicketEvent, StartContinuousRenewEvent } from "./event"

import { AuthTicket } from "../kernel/data"

export interface SaveAuthTicketMethod {
    <S>(auth: AuthTicket, post: Post<SaveAuthTicketEvent, S>): Promise<S>
}

export interface StartContinuousRenewMethod {
    <S>(post: Post<StartContinuousRenewEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
