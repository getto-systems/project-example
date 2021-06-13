import { ApplicationAbstractStateAction } from "../../../../../ui/vendor/getto-application/action/impl"

import { loadMenu } from "../../load_menu/method"
import { updateMenuBadge } from "../../update_menu_badge/impl/core"
import { hideMenuExpand, showMenuExpand } from "../../toggle_menu_expand/impl/core"

import { LoadMenuInfra, LoadMenuStore } from "../../load_menu/infra"
import { UpdateMenuBadgeInfra, UpdateMenuBadgeStore } from "../../update_menu_badge/infra"
import { ToggleMenuExpandInfra, ToggleMenuExpandStore } from "../../toggle_menu_expand/infra"

import {
    LoadMenuCoreMaterial,
    LoadMenuCoreAction,
    LoadMenuCoreState,
    initialLoadMenuCoreState,
} from "./action"

import { LoadMenuDetecter } from "../../kernel/method"

import { MenuCategoryPath } from "../../kernel/data"
import { initMenuBadgeStore, initMenuExpandStore } from "../../kernel/infra/store"

export type LoadMenuCoreInfra = LoadMenuInfra & UpdateMenuBadgeInfra & ToggleMenuExpandInfra

type Store = LoadMenuStore & UpdateMenuBadgeStore & ToggleMenuExpandStore

export function initLoadMenuCoreMaterial(
    infra: LoadMenuCoreInfra,
    detecter: LoadMenuDetecter,
): LoadMenuCoreMaterial {
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

export function initLoadMenuCoreAction(material: LoadMenuCoreMaterial): LoadMenuCoreAction {
    return new Action(material)
}

class Action
    extends ApplicationAbstractStateAction<LoadMenuCoreState>
    implements LoadMenuCoreAction {
    readonly initialState = initialLoadMenuCoreState

    material: LoadMenuCoreMaterial

    constructor(material: LoadMenuCoreMaterial) {
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

    updateBadge(): Promise<LoadMenuCoreState> {
        return this.material.updateBadge(this.post)
    }

    show(path: MenuCategoryPath): Promise<LoadMenuCoreState> {
        return this.material.show(path, this.post)
    }
    hide(path: MenuCategoryPath): Promise<LoadMenuCoreState> {
        return this.material.hide(path, this.post)
    }
}
