import { ApplicationMockStateAction } from "../../../../ui/vendor/getto-application/action/mock"

import { Menu } from "../kernel/data"
import { appendMenuCategoryPath, toMenuCategory, toMenuItem } from "../kernel/convert"

import { initialLoadMenuState, LoadMenuAction, LoadMenuState } from "./action"

export function mockLoadMenuAction(menu: Menu): LoadMenuAction {
    return new Action(menu)
}

class Action extends ApplicationMockStateAction<LoadMenuState> implements LoadMenuAction {
    readonly initialState = initialLoadMenuState

    constructor(menu: Menu) {
        super(async () => ({ type: "succeed-to-load", menu }))
    }

    async updateBadge(): Promise<LoadMenuState> {
        return this.initialState
    }
    async show(): Promise<LoadMenuState> {
        return this.initialState
    }
    async hide(): Promise<LoadMenuState> {
        return this.initialState
    }
}

export function mockMenu_home(): Menu {
    return mockMenu("ホーム", "lnir lnir-home", 10)
}
export function mockMenu(label: string, icon: string, badgeCount: number): Menu {
    const category = { label: "MAIN", permission: { type: "allow" } } as const
    return [
        {
            type: "category",
            isExpand: true,
            badgeCount,
            category: toMenuCategory(category),
            path: appendMenuCategoryPath([], category),
            children: [
                {
                    type: "item",
                    isActive: true,
                    badgeCount,
                    item: toMenuItem({ label, icon, path: "/index.html" }, "1.0.0"),
                },
            ],
        },
    ]
}
