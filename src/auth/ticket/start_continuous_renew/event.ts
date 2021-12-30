import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RenewAuthTicketError } from "../kernel/data"

export type SaveAuthProfileEvent =
    | Readonly<{ type: "succeed-to-save" }>
    | Readonly<{ type: "failed-to-save"; err: RepositoryError }>

export type StartContinuousRenewEvent =
    | Readonly<{ type: "succeed-to-start-continuous-renew"; continue: true }>
    | Readonly<{ type: "authn-not-expired"; continue: true }>
    | Readonly<{ type: "succeed-to-renew"; continue: true }>
    | Readonly<{ type: "required-to-login"; continue: false }>
    | Readonly<{ type: "failed-to-renew"; continue: false; err: RenewAuthTicketError }>
    | Readonly<{ type: "repository-error"; continue: false; err: RepositoryError }>
