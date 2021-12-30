import { env } from "../../../y_environment/ui/env"

import { newAuthProfileRepository } from "../../../auth/ticket/kernel/init/profile_repository"
import { newGetMenuBadgeRemote } from "../kernel/init/get_menu_badge_remote/fetch"
import { newGetMenuBadgeNoopRemote } from "../kernel/init/get_menu_badge_remote/noop"

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
        profileRepository: newAuthProfileRepository(feature),
        getMenuBadgeRemote: menuContent.loadMenuBadge
            ? newGetMenuBadgeRemote(feature)
            : newGetMenuBadgeNoopRemote(),
    }
}
