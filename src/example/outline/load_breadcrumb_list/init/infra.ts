import { env } from "../../../../y_environment/ui/env"

import { MenuContent } from "../../kernel/infra"

import { LoadBreadcrumbListInfra } from "../action"

export function newLoadBreadcrumbListInfra(menuContent: MenuContent): LoadBreadcrumbListInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}
