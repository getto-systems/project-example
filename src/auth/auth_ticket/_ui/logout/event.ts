import { RepositoryError } from "../../../../z_details/_ui/repository/data"
import { LogoutError } from "./data"

export type LogoutEvent =
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
    | Readonly<{ type: "failed-to-logout"; err: LogoutError }>
    | Readonly<{ type: "succeed-to-logout" }>
