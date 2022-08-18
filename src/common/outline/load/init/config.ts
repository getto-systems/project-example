import { env } from "../../../../y_environment/ui/env"

import { OutlineBreadcrumbListConfig, OutlineMenuConfig } from "../action"

import { MenuContent } from "../infra"

export function newOutlineBreadcrumbListConfig(
    menuContent: MenuContent,
): OutlineBreadcrumbListConfig {
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
