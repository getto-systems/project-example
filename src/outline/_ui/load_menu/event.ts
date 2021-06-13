import { RepositoryError } from "../../../z_details/_ui/repository/data"
import { Menu } from "../kernel/data"

export type LoadMenuEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "succeed-to-load"; menu: Menu }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
