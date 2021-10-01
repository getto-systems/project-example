import { env } from "../../../y_environment/ui/env"

import { newAuthzRepository } from "../../../auth/ticket/kernel/init/repository/authz"
import { newGetMenuBadgeRemote } from "../kernel/init/remote/get_menu_badge/fetch"
import { newGetMenuBadgeNoopRemote } from "../kernel/init/remote/get_menu_badge/noop"

import { RepositoryOutsideFeature } from "../../../z_lib/ui/repository/feature"
import { RemoteOutsideFeature } from "../../../z_lib/ui/remote/feature"

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
        authz: newAuthzRepository(feature),
        getMenuBadge: menuContent.loadMenuBadge
            ? newGetMenuBadgeRemote(feature)
            : newGetMenuBadgeNoopRemote(),
    }
}