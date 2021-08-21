import { newLoadMenuLocationDetecter } from "../kernel/init"
import { newLoadMenuInfra } from "../load_menu/init"
import { newUpdateMenuBadgeInfra } from "../update_menu_badge/init"
import { newToggleMenuExpandInfra } from "../toggle_menu_expand/init"

import { initLoadMenuCoreAction, initLoadMenuCoreMaterial } from "./core/impl"

import { MenuContent } from "../kernel/infra"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"
import { LocationOutsideFeature } from "../../../../z_details/_ui/location/feature"

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
