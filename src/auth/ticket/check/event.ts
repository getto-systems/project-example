import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { AuthTicket, RenewAuthTicketError } from "../kernel/data"

export type CheckAuthTicketEvent = Readonly<{ type: "try-to-instant-load" }> | RenewAuthTicketEvent

export type RenewAuthTicketEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "try-to-renew" }>
    | Readonly<{ type: "take-longtime-to-renew" }>
    | Readonly<{ type: "failed-to-renew"; err: RenewAuthTicketError }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
    | Readonly<{ type: "succeed-to-renew"; auth: AuthTicket }>
