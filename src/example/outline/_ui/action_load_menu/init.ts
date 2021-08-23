import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { loadMenu } from "../load_menu/method"
import { updateMenuBadge } from "../update_menu_badge/method"
import { hideMenuExpand, showMenuExpand } from "../toggle_menu_expand/method"

import { LoadMenuInfra, LoadMenuStore } from "../load_menu/infra"
import { UpdateMenuBadgeInfra, UpdateMenuBadgeStore } from "../update_menu_badge/infra"
import { ToggleMenuExpandInfra, ToggleMenuExpandStore } from "../toggle_menu_expand/infra"

import {
    LoadMenuMaterial,
    LoadMenuAction,
    LoadMenuState,
    initialLoadMenuState,
} from "./action"

import { LoadMenuDetecter } from "../kernel/method"

import { MenuCategoryPath } from "../kernel/data"
import { initMenuBadgeStore, initMenuExpandStore } from "../kernel/init/store"

export type LoadMenuActionInfra = LoadMenuInfra & UpdateMenuBadgeInfra & ToggleMenuExpandInfra

type Store = LoadMenuStore & UpdateMenuBadgeStore & ToggleMenuExpandStore

export function initLoadMenuMaterial(
    infra: LoadMenuActionInfra,
    detecter: LoadMenuDetecter,
): LoadMenuMaterial {
    const store: Store = {
        menuExpand: initMenuExpandStore(),
        menuBadge: initMenuBadgeStore(),
    }
    return {
        load: loadMenu(infra, store)(detecter),
        updateBadge: updateMenuBadge(infra, store)(detecter),
        show: showMenuExpand(infra, store)(detecter),
        hide: hideMenuExpand(infra, store)(detecter),
    }
}

export function initLoadMenuAction(material: LoadMenuMaterial): LoadMenuAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<LoadMenuState>
    implements LoadMenuAction {
    readonly initialState = initialLoadMenuState

    material: LoadMenuMaterial

    constructor(material: LoadMenuMaterial) {
        super(async () =>
            this.material.load((event) => {
                const state = this.post(event)

                switch (event.type) {
                    case "succeed-to-load":
                        // 初期ロード完了で最初の badge 更新を行う
                        return this.updateBadge()

                    default:
                        return state
                }
            }),
        )
        this.material = material
    }

    updateBadge(): Promise<LoadMenuState> {
        return this.material.updateBadge(this.post)
    }

    show(path: MenuCategoryPath): Promise<LoadMenuState> {
        return this.material.show(path, this.post)
    }
    hide(path: MenuCategoryPath): Promise<LoadMenuState> {
        return this.material.hide(path, this.post)
    }
}
