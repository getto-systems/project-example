import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { LoadMenuMethod } from "../../load_menu/method"
import { UpdateMenuBadgeMethod } from "../../update_menu_badge/method"
import { ToggleMenuExpandMethod } from "../../toggle_menu_expand/method"

import { LoadMenuEvent } from "../../load_menu/event"
import { UpdateMenuBadgeEvent } from "../../update_menu_badge/event"
import { ToggleMenuExpandEvent } from "../../toggle_menu_expand/event"

import { MenuCategoryPath } from "../../kernel/data"

export interface LoadMenuCoreAction extends ApplicationStateAction<LoadMenuCoreState> {
    updateBadge(): Promise<LoadMenuCoreState>
    show(path: MenuCategoryPath): Promise<LoadMenuCoreState>
    hide(path: MenuCategoryPath): Promise<LoadMenuCoreState>
}

export type LoadMenuCoreMaterial = Readonly<{
    load: LoadMenuMethod
    updateBadge: UpdateMenuBadgeMethod
    show: ToggleMenuExpandMethod
    hide: ToggleMenuExpandMethod
}>

export type LoadMenuCoreState =
    | Readonly<{ type: "initial-menu" }>
    | LoadMenuEvent
    | UpdateMenuBadgeEvent
    | ToggleMenuExpandEvent

export const initialLoadMenuCoreState: LoadMenuCoreState = { type: "initial-menu" }
