import { env } from "../../../../y_environment/_ui/env"

import { newAuthzRepository } from "../../../../auth/auth_ticket/_ui/kernel/init/repository/authz"
import { newMenuExpandRepository } from "../kernel/init/repository/menu_expand"

import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"

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
