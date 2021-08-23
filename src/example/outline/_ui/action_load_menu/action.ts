import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { LoadMenuMethod } from "../load_menu/method"
import { UpdateMenuBadgeMethod } from "../update_menu_badge/method"
import { ToggleMenuExpandMethod } from "../toggle_menu_expand/method"

import { LoadMenuEvent } from "../load_menu/event"
import { UpdateMenuBadgeEvent } from "../update_menu_badge/event"
import { ToggleMenuExpandEvent } from "../toggle_menu_expand/event"

import { MenuCategoryPath } from "../kernel/data"

export interface LoadMenuAction extends ApplicationStateAction<LoadMenuState> {
    updateBadge(): Promise<LoadMenuState>
    show(path: MenuCategoryPath): Promise<LoadMenuState>
    hide(path: MenuCategoryPath): Promise<LoadMenuState>
}

export type LoadMenuMaterial = Readonly<{
    load: LoadMenuMethod
    updateBadge: UpdateMenuBadgeMethod
    show: ToggleMenuExpandMethod
    hide: ToggleMenuExpandMethod
}>

export type LoadMenuState =
    | Readonly<{ type: "initial-menu" }>
    | LoadMenuEvent
    | UpdateMenuBadgeEvent
    | ToggleMenuExpandEvent

export const initialLoadMenuState: LoadMenuState = { type: "initial-menu" }
