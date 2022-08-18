import { env } from "../../../../y_environment/ui/env"

import { LoadBreadcrumbListConfig, OutlineMenuConfig } from "../action"

import { MenuContent } from "../infra"

export function newLoadBreadcrumbListConfig(menuContent: MenuContent): LoadBreadcrumbListConfig {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}

export function newOutlineMenuConfig(menuContent: MenuContent): OutlineMenuConfig {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}
