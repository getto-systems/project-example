import { env } from "../../../y_environment/_ui/env"

import { newAuthzRepositoryPod } from "../../../auth/auth_ticket/_ui/kernel/infra/repository/authz"
import { newGetMenuBadgeRemote } from "../kernel/infra/remote/get_menu_badge/fetch"
import { newGetMenuBadgeNoopRemote } from "../kernel/infra/remote/get_menu_badge/noop"

import { RepositoryOutsideFeature } from "../../../z_details/_ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_details/_ui/remote/feature"

import { UpdateMenuBadgeInfra } from "./infra"
import { MenuContent } from "../kernel/infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature
export function newUpdateMenuBadgeInfra(
    feature: OutsideFeature,
    menuContent: MenuContent,
): UpdateMenuBadgeInfra {
    return {
        version: env.version,
        menuTree: menuContent.menuTree,
        authz: newAuthzRepositoryPod(feature),
        getMenuBadge: menuContent.loadMenuBadge
            ? newGetMenuBadgeRemote(feature)
            : newGetMenuBadgeNoopRemote(),
    }
}
