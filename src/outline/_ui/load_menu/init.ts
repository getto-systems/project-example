import { env } from "../../../y_environment/_ui/env"

import { newAuthzRepository } from "../../../auth/auth_ticket/_ui/kernel/infra/repository/authz"
import { newMenuExpandRepositoryPod } from "../kernel/infra/repository/menu_expand"

import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"

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
        menuExpand: newMenuExpandRepositoryPod(feature, menuContent),
    }
}
