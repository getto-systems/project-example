import { env } from "../../../y_environment/_ui/env"

import { newAuthzRepositoryPod } from "../../../auth/auth_ticket/_ui/kernel/infra/repository/authz"
import { newMenuExpandRepositoryPod } from "../kernel/infra/repository/menu_expand"

import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"

import { MenuContent } from "../kernel/infra"
import { ToggleMenuExpandInfra } from "./infra"

export function newToggleMenuExpandInfra(
    feature: RepositoryOutsideFeature,
    menuContent: MenuContent,
): ToggleMenuExpandInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        authz: newAuthzRepositoryPod(feature),
        menuExpand: newMenuExpandRepositoryPod(feature, menuContent),
    }
}
