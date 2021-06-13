import { env } from "../../../y_environment/_ui/env"

import { newAuthzRepositoryPod } from "../../../auth/auth_ticket/_ui/kernel/infra/repository/authz"
import { newMenuExpandRepositoryPod } from "../kernel/infra/repository/menu_expand"

import { MenuContent } from "../kernel/infra"
import { LoadMenuInfra } from "./infra"
import { RepositoryOutsideFeature } from "../../../../ui/vendor/getto-application/infra/repository/infra"

export function newLoadMenuInfra(
    feature: RepositoryOutsideFeature,
    menuContent: MenuContent,
): LoadMenuInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        authz: newAuthzRepositoryPod(feature),
        menuExpand: newMenuExpandRepositoryPod(feature, menuContent),
    }
}
