import { newLoadMenuLocationDetecter } from "../kernel/init"
import { newLoadMenuInfra } from "../load_menu/impl/init"
import { newUpdateMenuBadgeInfra } from "../update_menu_badge/impl/init"
import { newToggleMenuExpandInfra } from "../toggle_menu_expand/impl/init"

import { initLoadMenuCoreAction, initLoadMenuCoreMaterial } from "./core/impl"

import { MenuContent } from "../kernel/infra"

import { RemoteOutsideFeature } from "../../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../../ui/vendor/getto-application/infra/repository/infra"
import { LocationOutsideFeature } from "../../../../ui/vendor/getto-application/location/infra"

import { LoadMenuResource } from "./resource"

export function newLoadMenuResource(
    feature: RemoteOutsideFeature & RepositoryOutsideFeature & LocationOutsideFeature,
    menuContent: MenuContent,
): LoadMenuResource {
    return {
        menu: initLoadMenuCoreAction(
            initLoadMenuCoreMaterial(
                {
                    ...newLoadMenuInfra(feature, menuContent),
                    ...newUpdateMenuBadgeInfra(feature, menuContent),
                    ...newToggleMenuExpandInfra(feature, menuContent),
                },
                newLoadMenuLocationDetecter(feature),
            ),
        ),
    }
}
