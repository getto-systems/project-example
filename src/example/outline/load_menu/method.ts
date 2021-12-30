import { buildMenu } from "../kernel/helper"

import { initMenuExpand, MenuBadge } from "../kernel/infra"
import { LoadMenuInfra, LoadMenuStore } from "./infra"

import { LoadMenuEvent } from "./event"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { MenuTargetPath } from "../kernel/data"

export interface LoadMenuMethod {
    <S>(
        menuTargetPath: ConvertLocationResult<MenuTargetPath>,
        post: Post<LoadMenuEvent, S>,
    ): Promise<S>
}

interface Load {
    (infra: LoadMenuInfra, store: LoadMenuStore): LoadMenuMethod
}
export const loadMenu: Load = (infra, store) => async (menuTargetPath, post) => {
    const { version, menuTree, profileRepository, menuExpandRepository } = infra

    const profileResult = await profileRepository.get()
    if (!profileResult.success) {
        return post({ type: "repository-error", err: profileResult.err })
    }
    if (!profileResult.found) {
        return post({ type: "required-to-login" })
    }

    const menuExpandResult = await menuExpandRepository.get()
    if (!menuExpandResult.success) {
        return post({ type: "repository-error", err: menuExpandResult.err })
    }

    const expand = menuExpandResult.found ? menuExpandResult.value : initMenuExpand()

    // update badge と toggle のため、現在の expand を保存しておく必要がある
    store.menuExpand.set(expand)

    return post({
        type: "succeed-to-load",
        menu: buildMenu({
            version,
            menuTree,
            menuTargetPath,
            profile: profileResult.value,
            menuExpand: expand,
            menuBadge: EMPTY_BADGE, // ロードに時間がかかる可能性があるのであとでロードする
        }),
    })
}

const EMPTY_BADGE: MenuBadge = new Map()

interface Post<E, S> {
    (event: E): S
}
