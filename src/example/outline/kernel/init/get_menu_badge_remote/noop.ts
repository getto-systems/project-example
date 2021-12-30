import { GetMenuBadgeRemote, MenuBadge } from "../../infra"

export function newGetMenuBadgeNoopRemote(): GetMenuBadgeRemote {
    return async () => ({ success: true, value: EMPTY_BADGE })
}

const EMPTY_BADGE: MenuBadge = new Map()
