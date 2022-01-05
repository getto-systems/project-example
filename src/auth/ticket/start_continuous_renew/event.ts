import { RemoteCommonError } from "../../../z_lib/ui/remote/data"
import { RepositoryError } from "../../../z_lib/ui/repository/data"

export type StartContinuousRenewEvent =
    | Readonly<{ type: "succeed-to-start-continuous-renew"; continue: true }>
    | Readonly<{ type: "ticket-not-expired"; continue: true }>
    | Readonly<{ type: "succeed-to-renew"; continue: true }>
    | Readonly<{ type: "required-to-login"; continue: false }>
    | Readonly<{ type: "failed-to-renew"; continue: false; err: RemoteCommonError }>
    | Readonly<{ type: "repository-error"; continue: false; err: RepositoryError }>
