import { RepositoryError } from "../../../z_details/_ui/repository/data"
import { Menu } from "../kernel/data"

export type ToggleMenuExpandEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "succeed-to-toggle"; menu: Menu }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
