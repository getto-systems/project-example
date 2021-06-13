import { authzRepositoryConverter } from "../../../../auth/auth_ticket/_ui/kernel/converter"
import { menuExpandRepositoryConverter } from "../../kernel/converter"

import { buildMenu } from "../../kernel/helper"

import { LoadMenuInfra, LoadMenuStore } from "../infra"
import { initMenuExpand, MenuBadge } from "../../kernel/infra"

import { LoadMenuPod } from "../method"

interface Load {
    (infra: LoadMenuInfra, store: LoadMenuStore): LoadMenuPod
}
export const loadMenu: Load = (infra, store) => (detecter) => async (post) => {
    const authz = infra.authz(authzRepositoryConverter)
    const menuExpand = infra.menuExpand(menuExpandRepositoryConverter)

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

    const menuExpandResult = await menuExpand.get()
    if (!menuExpandResult.success) {
        return post({ type: "repository-error", err: menuExpandResult.err })
    }

    const expand = menuExpandResult.found ? menuExpandResult.value : initMenuExpand()

    // update badge と toggle のため、現在の expand を保存しておく必要がある
    store.menuExpand.set(expand)

    return post({
        type: "succeed-to-load",
        menu: buildMenu({
            version: infra.version,
            menuTree: infra.menuTree,
            menuTargetPath: detecter(),
            grantedRoles: authzResult.value.roles,
            menuExpand: expand,
            menuBadge: EMPTY_BADGE, // ロードに時間がかかる可能性があるのであとでロードする
        }),
    })
}

const EMPTY_BADGE: MenuBadge = new Map()
