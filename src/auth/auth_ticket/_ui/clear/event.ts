import { RepositoryError } from "../../../../z_details/_ui/repository/data"
import { ClearAuthTicketError } from "./data"

export type ClearAuthTicketEvent =
    | Readonly<{ type: "failed-to-logout"; err: RepositoryError }> // TODO これは repository-error にするべき
    | Readonly<{ type: "failed-to-clear"; err: ClearAuthTicketError }>
    | Readonly<{ type: "succeed-to-logout" }>
