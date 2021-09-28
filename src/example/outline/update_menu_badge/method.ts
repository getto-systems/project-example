import { buildMenu, BuildMenuParams } from "../kernel/helper"

import { initMenuExpand, MenuBadge } from "../kernel/infra"
import { UpdateMenuBadgeInfra, UpdateMenuBadgeStore } from "./infra"

import { UpdateMenuBadgeEvent } from "./event"
import { ConvertLocationResult } from "../../../z_details/_ui/location/data"
import { MenuTargetPath } from "../kernel/data"

export interface UpdateMenuBadgeMethod {
    <S>(
        menuTargetPath: ConvertLocationResult<MenuTargetPath>,
        post: Post<UpdateMenuBadgeEvent, S>,
    ): Promise<S>
}

interface Update {
    (infra: UpdateMenuBadgeInfra, store: UpdateMenuBadgeStore): UpdateMenuBadgeMethod
}
export const updateMenuBadge: Update = (infra, store) => async (menuTargetPath, post) => {
    const { authz } = infra

    const authzResult = await authz.get()
    if (!authzResult.success) {
        return post({ type: "repository-error", err: authzResult.err })
    }
    if (!authzResult.found) {
        const authzRemoveResult = await authz.remove()
        if (!authzRemoveResult.success) {
            return post({ type: "repository-error", err: authzRemoveResult.err })
        }
        return post({ type: "required-to-login" })
    }

    const fetchResult = store.menuExpand.get()
    const expand = fetchResult.found ? fetchResult.value : initMenuExpand()

    const buildParams: BuildMenuParams = {
        version: infra.version,
        grantedRoles: authzResult.value.roles,
        menuExpand: expand,
        menuTargetPath,
        menuTree: infra.menuTree,
        menuBadge: EMPTY_BADGE,
    }

    const response = await infra.getMenuBadge()
    if (!response.success) {
        return post({ type: "failed-to-update", menu: buildMenu(buildParams), err: response.err })
    }

    store.menuBadge.set(response.value)

    return post({
        type: "succeed-to-update",
        menu: buildMenu({ ...buildParams, menuBadge: response.value }),
    })
}

const EMPTY_BADGE: MenuBadge = new Map()

interface Post<E, S> {
    (event: E): S
}
