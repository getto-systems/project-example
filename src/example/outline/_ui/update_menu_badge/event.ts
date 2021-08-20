import { RepositoryError } from "../../../../z_details/_ui/repository/data"
import { GetMenuBadgeError, Menu } from "../kernel/data"

export type UpdateMenuBadgeEvent =
    | Readonly<{ type: "succeed-to-update"; menu: Menu }>
    | Readonly<{ type: "failed-to-update"; menu: Menu; err: GetMenuBadgeError }>
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>
