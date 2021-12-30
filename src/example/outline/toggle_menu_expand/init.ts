import { env } from "../../../y_environment/ui/env"

import { newAuthProfileRepository } from "../../../auth/ticket/kernel/init/profile_repository"
import { newMenuExpandRepository } from "../kernel/init/menu_expand_repository"

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
        profileRepository: newAuthProfileRepository(feature),
        menuExpandRepository: newMenuExpandRepository(feature, menuContent),
    }
}
