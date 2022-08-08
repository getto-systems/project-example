import {
    ApplicationState,
    initApplicationState,
} from "../../../z_vendor/getto-application/action/action"

import { buildMenu, BuildMenuParams } from "./helper"
import { toMenuCategory, toMenuItem } from "./convert"

import {
    initMenuExpand,
    MenuBadge,
    MenuTargetPathDetecter,
    MenuTree,
    MenuTreeCategory,
    MenuTreeItem,
    MenuTreeNode,
} from "./infra"
import { LoadMenuBadgeRemote, MenuBadgeStore, MenuExpandStore, MenuExpandRepository } from "./infra"
import { AuthTicketRepository } from "../../../auth/ticket/kernel/infra"

import { RepositoryError } from "../../../z_lib/ui/repository/data"
import { RemoteCommonError } from "../../../z_lib/ui/remote/data"
import { BreadcrumbList, BreadcrumbNode, Menu, MenuCategoryPath, MenuTargetPath } from "./data"

export interface LoadBreadcrumbListAction {
    load(): BreadcrumbList
}

export type LoadBreadcrumbListMaterial = Readonly<{
    shell: LoadBreadcrumbListShell
    config: LoadBreadcrumbListConfig
}>
export type LoadBreadcrumbListShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>
export type LoadBreadcrumbListConfig = Readonly<{
    version: string
    menuTree: MenuTree
}>

export function initLoadBreadcrumbListAction(
    material: LoadBreadcrumbListMaterial,
): LoadBreadcrumbListAction {
    return {
        load: () => load(material),
    }
}

function load({ shell, config }: LoadBreadcrumbListMaterial): BreadcrumbList {
    const { detectTargetPath } = shell

    const menuTargetPath = detectTargetPath()
    if (!menuTargetPath.valid) {
        return EMPTY
    }
    return build(menuTargetPath.value)

    function build(currentPath: MenuTargetPath): BreadcrumbList {
        return toBreadcrumb(config.menuTree)

        function toBreadcrumb(tree: MenuTree): BreadcrumbList {
            for (let i = 0; i < tree.length; i++) {
                const breadcrumbList = findFocusedNode(tree[i])
                if (breadcrumbList.length > 0) {
                    return breadcrumbList
                }
            }
            return EMPTY
        }
        function findFocusedNode(node: MenuTreeNode): readonly BreadcrumbNode[] {
            switch (node.type) {
                case "category":
                    return categoryNode(node.category, node.children)
                case "item":
                    return itemNode(node.item)
            }
        }
        function categoryNode(
            category: MenuTreeCategory,
            children: MenuTree,
        ): readonly BreadcrumbNode[] {
            const breadcrumb = toBreadcrumb(children)
            if (breadcrumb.length === 0) {
                return EMPTY
            }
            return [{ type: "category", category: toMenuCategory(category) }, ...breadcrumb]
        }
        function itemNode(item: MenuTreeItem): readonly BreadcrumbNode[] {
            if (item.path !== currentPath) {
                return EMPTY
            }
            return [{ type: "item", item: toMenuItem(item, config.version) }]
        }
    }
}

const EMPTY: BreadcrumbList = []

export interface LoadMenuAction {
    readonly state: ApplicationState<LoadMenuState>
    updateBadge(): Promise<LoadMenuState>
    show(path: MenuCategoryPath): Promise<LoadMenuState>
    hide(path: MenuCategoryPath): Promise<LoadMenuState>
}

export type LoadMenuState =
    | Readonly<{ type: "initial-menu" }>
    | LoadMenuEvent
    | UpdateMenuBadgeEvent
    | ToggleMenuExpandEvent

const initialState: LoadMenuState = { type: "initial-menu" }

export type LoadMenuMaterial = Readonly<{
    infra: LoadMenuInfra
    shell: LoadMenuShell
    config: LoadMenuConfig
}>
export type LoadMenuInfra = Readonly<{
    loadMenuBadgeRemote: LoadMenuBadgeRemote
    ticketRepository: AuthTicketRepository
    menuExpandRepository: MenuExpandRepository
    menuExpandStore: MenuExpandStore
    menuBadgeStore: MenuBadgeStore
}>
export type LoadMenuShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>
export type LoadMenuConfig = Readonly<{
    version: string
    menuTree: MenuTree
}>

export function initLoadMenuAction(material: LoadMenuMaterial): LoadMenuAction {
    return new Action(material)
}

class Action implements LoadMenuAction {
    readonly material: LoadMenuMaterial
    readonly state: ApplicationState<LoadMenuState>
    readonly post: (state: LoadMenuState) => LoadMenuState

    constructor(material: LoadMenuMaterial) {
        const { state, post } = initApplicationState({
            initialState,
            ignite: () => this.load(),
        })
        this.material = material
        this.state = state
        this.post = post
    }
    async load(): Promise<LoadMenuState> {
        return loadMenu(this.material, (event) => {
            const state = this.post(event)

            switch (event.type) {
                case "succeed-to-load":
                    // 初期ロード完了で最初の badge 更新を行う
                    return this.updateBadge()

                default:
                    return state
            }
        })
    }

    updateBadge(): Promise<LoadMenuState> {
        return updateMenuBadge(this.material, this.post)
    }

    show(path: MenuCategoryPath): Promise<LoadMenuState> {
        return toggleMenuExpand(this.material, path, true, this.post)
    }
    hide(path: MenuCategoryPath): Promise<LoadMenuState> {
        return toggleMenuExpand(this.material, path, false, this.post)
    }
}

type LoadMenuEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "succeed-to-load"; menu: Menu }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

async function loadMenu<S>(
    { infra, shell, config }: LoadMenuMaterial,
    post: Post<LoadMenuEvent, S>,
): Promise<S> {
    const { ticketRepository, menuExpandRepository, menuExpandStore } = infra

    const ticketResult = await ticketRepository.get()
    if (!ticketResult.success) {
        return post({ type: "repository-error", err: ticketResult.err })
    }
    if (!ticketResult.found) {
        return post({ type: "required-to-login" })
    }

    const menuExpandResult = await menuExpandRepository.get()
    if (!menuExpandResult.success) {
        return post({ type: "repository-error", err: menuExpandResult.err })
    }

    const expand = menuExpandResult.found ? menuExpandResult.value : initMenuExpand()

    // update badge と toggle のため、現在の expand を保存しておく必要がある
    menuExpandStore.set(expand)

    return post({
        type: "succeed-to-load",
        menu: buildMenu({
            version: config.version,
            menuTree: config.menuTree,
            menuTargetPath: shell.detectTargetPath(),
            ticket: ticketResult.value,
            menuExpand: expand,
            menuBadge: EMPTY_BADGE, // ロードに時間がかかる可能性があるのであとでロードする
        }),
    })
}

type ToggleMenuExpandEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "succeed-to-toggle"; menu: Menu }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

async function toggleMenuExpand<S>(
    { infra, shell, config }: LoadMenuMaterial,
    path: MenuCategoryPath,
    isShow: boolean,
    post: Post<ToggleMenuExpandEvent, S>,
): Promise<S> {
    const { ticketRepository, menuExpandRepository, menuExpandStore, menuBadgeStore } = infra

    const ticketResult = await ticketRepository.get()
    if (!ticketResult.success) {
        return post({ type: "repository-error", err: ticketResult.err })
    }
    if (!ticketResult.found) {
        return post({ type: "required-to-login" })
    }

    const fetchMenuExpandResult = menuExpandStore.get()
    const expand = fetchMenuExpandResult.found ? fetchMenuExpandResult.value : initMenuExpand()

    if (isShow) {
        expand.register(path)
    } else {
        expand.remove(path)
    }

    // 別なタブで expand を変更した場合は上書き合戦になるが、マージは大変なのでさぼる
    // 対応が必要になったらストレージに update を追加してトランザクション内でマージする必要がある
    const storeResult = await menuExpandRepository.set(expand)
    if (!storeResult.success) {
        return post({ type: "repository-error", err: storeResult.err })
    }

    menuExpandStore.set(expand)

    const fetchMenuBadgeResult = menuBadgeStore.get()
    const badge = fetchMenuBadgeResult.found ? fetchMenuBadgeResult.value : EMPTY_BADGE

    return post({
        type: "succeed-to-toggle",
        menu: buildMenu({
            version: config.version,
            menuTree: config.menuTree,
            menuTargetPath: shell.detectTargetPath(),
            ticket: ticketResult.value,
            menuExpand: expand,
            menuBadge: badge,
        }),
    })
}

type UpdateMenuBadgeEvent =
    | Readonly<{ type: "succeed-to-update"; menu: Menu }>
    | Readonly<{ type: "failed-to-update"; menu: Menu; err: RemoteCommonError }>
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

async function updateMenuBadge<S>(
    { infra, shell, config }: LoadMenuMaterial,
    post: Post<UpdateMenuBadgeEvent, S>,
): Promise<S> {
    const { ticketRepository, loadMenuBadgeRemote, menuExpandStore, menuBadgeStore } = infra

    const ticketResult = await ticketRepository.get()
    if (!ticketResult.success) {
        return post({ type: "repository-error", err: ticketResult.err })
    }
    if (!ticketResult.found) {
        return post({ type: "required-to-login" })
    }

    const fetchResult = menuExpandStore.get()
    const expand = fetchResult.found ? fetchResult.value : initMenuExpand()

    const buildParams: BuildMenuParams = {
        version: config.version,
        menuTree: config.menuTree,
        ticket: ticketResult.value,
        menuExpand: expand,
        menuTargetPath: shell.detectTargetPath(),
        menuBadge: EMPTY_BADGE,
    }

    const response = await loadMenuBadgeRemote()
    if (!response.success) {
        return post({ type: "failed-to-update", menu: buildMenu(buildParams), err: response.err })
    }

    menuBadgeStore.set(response.value)

    return post({
        type: "succeed-to-update",
        menu: buildMenu({ ...buildParams, menuBadge: response.value }),
    })
}

const EMPTY_BADGE: MenuBadge = new Map()

interface Post<E, S> {
    (event: E): S
}
