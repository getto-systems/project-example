import { env } from "../../../y_environment/_ui/env"

import { MenuContent } from "../kernel/infra"
import { LoadBreadcrumbListInfra } from "./infra"

export function newLoadBreadcrumbListInfra(menuContent: MenuContent): LoadBreadcrumbListInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}
