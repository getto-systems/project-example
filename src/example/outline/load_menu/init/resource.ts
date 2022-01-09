import { LocationOutsideFeature } from "../../../../z_lib/ui/location/feature"
import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_lib/ui/repository/feature"

import { newLoadMenuShell } from "./shell"

import { initLoadMenuAction, LoadMenuAction } from "../action"

import { MenuContent } from "../../kernel/infra"
import { newLoadMenuInfra } from "./infra"

type OutsideFeature = RepositoryOutsideFeature & RemoteOutsideFeature & LocationOutsideFeature
export function newLoadMenuResource(
    feature: OutsideFeature,
    menuContent: MenuContent,
): Readonly<{ menu: LoadMenuAction }> {
    return {
        menu: initLoadMenuAction(newLoadMenuInfra(feature, menuContent), newLoadMenuShell(feature)),
    }
}
