import { env } from "../../../y_environment/ui/env"

import { newAuthzRepository } from "../../../auth/ticket/kernel/init/repository/authz"
import { newMenuExpandRepository } from "../kernel/init/repository/menu_expand"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"

import { MenuContent } from "../kernel/infra"
import { LoadMenuInfra } from "./infra"

export function newLoadMenuInfra(
    feature: RepositoryOutsideFeature,
    menuContent: MenuContent,
): LoadMenuInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        authz: newAuthzRepository(feature),
        menuExpand: newMenuExpandRepository(feature, menuContent),
    }
}
