import { env } from "../../../../y_environment/ui/env"

import { MenuContent } from "../../kernel/infra"

import { LoadBreadcrumbListConfig } from "../action"

export function newLoadBreadcrumbListConfig(menuContent: MenuContent): LoadBreadcrumbListConfig {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}
