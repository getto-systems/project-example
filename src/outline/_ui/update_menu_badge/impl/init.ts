import { env } from "../../../../y_environment/env"

import { newAuthzRepositoryPod } from "../../../../auth/auth_ticket/_ui/kernel/infra/repository/authz"
import { newGetMenuBadgeRemote } from "../../kernel/infra/remote/get_menu_badge/core"
import { newGetMenuBadgeNoopRemote } from "../../kernel/infra/remote/get_menu_badge/noop"

import { UpdateMenuBadgeInfra } from "../infra"
import { MenuContent } from "../../kernel/infra"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/infra"
import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/infra"

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
