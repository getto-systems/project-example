import { env } from "../../../y_environment/ui/env"

import { newAuthzRepository } from "../../../auth/ticket/kernel/init/repository/authz"
import { newMenuExpandRepository } from "../kernel/init/repository/menu_expand"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { MenuContent } from "../kernel/infra"
import { ToggleMenuExpandInfra } from "./infra"

export function newToggleMenuExpandInfra(
    feature: RepositoryOutsideFeature,
    menuContent: MenuContent,
): ToggleMenuExpandInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        authz: newAuthzRepository(feature),
        menuExpand: newMenuExpandRepository(feature, menuContent),
    }
}