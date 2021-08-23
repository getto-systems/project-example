import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/init"

import { initMenuBadgeStore, initMenuExpandStore } from "../kernel/init/store"

import { loadMenu } from "../load_menu/method"
import { updateMenuBadge } from "../update_menu_badge/method"
import { hideMenuExpand, showMenuExpand } from "../toggle_menu_expand/method"

import { LoadMenuInfra, LoadMenuStore } from "../load_menu/infra"
import { UpdateMenuBadgeInfra, UpdateMenuBadgeStore } from "../update_menu_badge/infra"
import { ToggleMenuExpandInfra, ToggleMenuExpandStore } from "../toggle_menu_expand/infra"

import { LoadMenuMaterial, LoadMenuAction, LoadMenuState, initialLoadMenuState } from "./action"

import { LoadMenuDetecter } from "../kernel/method"

import { MenuCategoryPath } from "../kernel/data"

export type LoadMenuActionInfra = LoadMenuInfra & UpdateMenuBadgeInfra & ToggleMenuExpandInfra

type Store = LoadMenuStore & UpdateMenuBadgeStore & ToggleMenuExpandStore

export function initLoadMenuMaterial(infra: LoadMenuActionInfra): LoadMenuMaterial {
    const store: Store = {
        menuExpand: initMenuExpandStore(),
        menuBadge: initMenuBadgeStore(),
    }
    return {
        load: loadMenu(infra, store),
        updateBadge: updateMenuBadge(infra, store),
        show: showMenuExpand(infra, store),
        hide: hideMenuExpand(infra, store),
    }
}

export function initLoadMenuAction(
    material: LoadMenuMaterial,
    detecter: LoadMenuDetecter,
): LoadMenuAction {
    return new Action(material, detecter)
}

class Action extends ApplicationAbstractStateAction<LoadMenuState> implements LoadMenuAction {
    readonly initialState = initialLoadMenuState

    material: LoadMenuMaterial
    detecter: LoadMenuDetecter

    constructor(material: LoadMenuMaterial, detecter: LoadMenuDetecter) {
        super(async () =>
            this.material.load(detecter(), (event) => {
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
        this.detecter = detecter
    }

    updateBadge(): Promise<LoadMenuState> {
        return this.material.updateBadge(this.detecter(), this.post)
    }

    show(path: MenuCategoryPath): Promise<LoadMenuState> {
        return this.material.show(this.detecter(), path, this.post)
    }
    hide(path: MenuCategoryPath): Promise<LoadMenuState> {
        return this.material.hide(this.detecter(), path, this.post)
    }
}
