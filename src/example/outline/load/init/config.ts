import { env } from "../../../../y_environment/ui/env"

import { LoadBreadcrumbListConfig, LoadMenuConfig } from "../action"

import { MenuContent } from "../infra"

export function newLoadBreadcrumbListConfig(menuContent: MenuContent): LoadBreadcrumbListConfig {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}

export function newLoadMenuConfig(menuContent: MenuContent): LoadMenuConfig {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
    }
}
