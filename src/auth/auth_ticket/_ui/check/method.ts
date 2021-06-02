import { RenewAuthTicketEvent, CheckAuthTicketEvent } from "./event"

export interface CheckAuthTicketMethod {
    <S>(post: Post<CheckAuthTicketEvent, S>): Promise<S>
}

export interface RenewAuthTicketMethod {
    <S>(post: Post<RenewAuthTicketEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
