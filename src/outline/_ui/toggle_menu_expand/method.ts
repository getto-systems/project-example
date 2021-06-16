import { buildMenu } from "../kernel/helper"

import { LoadMenuDetecter } from "../kernel/method"

import { ToggleMenuExpandEvent } from "./event"

import { initMenuExpand, MenuBadge, MenuExpand } from "../kernel/infra"
import { ToggleMenuExpandInfra, ToggleMenuExpandStore } from "./infra"

import { MenuCategoryPath } from "../kernel/data"

export interface ToggleMenuExpandPod {
    (detecter: LoadMenuDetecter): ToggleMenuExpandMethod
}
export interface ToggleMenuExpandMethod {
    <S>(path: MenuCategoryPath, post: Post<ToggleMenuExpandEvent, S>): Promise<S>
}

interface Toggle {
    (infra: ToggleMenuExpandInfra, store: ToggleMenuExpandStore): ToggleMenuExpandPod
}

export const showMenuExpand: Toggle = modifyMenuExpand((expand, path) => expand.register(path))
export const hideMenuExpand: Toggle = modifyMenuExpand((expand, path) => expand.remove(path))

interface ModifyExpand {
    (expand: MenuExpand, path: MenuCategoryPath): void
}
function modifyMenuExpand(modify: ModifyExpand): Toggle {
    return (infra, store) => (detecter) => async (path, post) => {
        const { authz, menuExpand } = infra

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

        const fetchMenuExpandResult = store.menuExpand.get()
        const expand = fetchMenuExpandResult.found ? fetchMenuExpandResult.value : initMenuExpand()

        modify(expand, path)

        // 別なタブで expand を変更した場合は上書き合戦になるが、マージは大変なのでさぼる
        // 対応が必要になったらストレージに update を追加してトランザクション内でマージする必要がある
        const storeResult = await menuExpand.set(expand)
        if (!storeResult.success) {
            return post({ type: "repository-error", err: storeResult.err })
        }

        store.menuExpand.set(expand)

        const fetchMenuBadgeResult = store.menuBadge.get()
        const badge = fetchMenuBadgeResult.found ? fetchMenuBadgeResult.value : EMPTY_BADGE

        return post({
            type: "succeed-to-toggle",
            menu: buildMenu({
                version: infra.version,
                menuTree: infra.menuTree,
                menuTargetPath: detecter(),
                grantedRoles: authzResult.value.roles,
                menuExpand: expand,
                menuBadge: badge,
            }),
        })
    }
}

const EMPTY_BADGE: MenuBadge = new Map()

interface Post<E, S> {
    (event: E): S
}
