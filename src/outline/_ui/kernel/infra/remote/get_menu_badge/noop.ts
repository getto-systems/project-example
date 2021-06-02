import { GetMenuBadgeRemotePod, MenuBadge } from "../../../infra"

export function newGetMenuBadgeNoopRemote(): GetMenuBadgeRemotePod {
    return () => async () => ({ success: true, value: EMPTY_BADGE })
}

const EMPTY_BADGE: MenuBadge = new Map()
