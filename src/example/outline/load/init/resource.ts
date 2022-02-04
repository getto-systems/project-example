import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newLoadMenuInfra } from "./infra"
import { newLoadMenuShell } from "./shell"
import { newLoadMenuConfig } from "./config"

import { initLoadMenuAction, LoadMenuAction } from "../action"

import { MenuContent } from "../../kernel/infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
export function newLoadMenuResource(
    feature: OutsideFeature,
    menuContent: MenuContent,
): Readonly<{ menu: LoadMenuAction }> {
    return {
        menu: initLoadMenuAction({
            infra: newLoadMenuInfra(feature, menuContent),
            shell: newLoadMenuShell(feature),
            config: newLoadMenuConfig(menuContent),
        }),
    }
}
