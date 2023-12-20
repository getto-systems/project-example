import { Atom, initAtom } from "../../../z_vendor/getto-atom/atom"

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

import { RepositoryError } from "../../util/repository/data"
import { RemoteCommonError } from "../../util/remote/data"
import { BreadcrumbList, BreadcrumbNode, Menu, MenuCategoryPath, MenuTargetPath } from "./data"

export interface OutlineBreadcrumbListAction {
    readonly state: Atom<OutlineBreadcrumbListState>
}

export type OutlineBreadcrumbListState = Readonly<{ list: BreadcrumbList }>

export type OutlineBreadcrumbListMaterial = Readonly<{
    shell: OutlineBreadcrumbListShell
    config: OutlineBreadcrumbListConfig
}>
export type OutlineBreadcrumbListShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>
export type OutlineBreadcrumbListConfig = Readonly<{
    version: string
    menuTree: MenuTree
}>

export function initOutlineBreadcrumbListAction(
    material: OutlineBreadcrumbListMaterial,
): OutlineBreadcrumbListAction {
    const { state } = initAtom({ initialState: { list: load(material) } })
    return {
        state,
    }
}

function load({ shell, config }: OutlineBreadcrumbListMaterial): BreadcrumbList {
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

export interface OutlineMenuAction {
    readonly state: Atom<OutlineMenuState>
    updateBadge(): Promise<OutlineMenuState>
    show(path: MenuCategoryPath): Promise<OutlineMenuState>
    hide(path: MenuCategoryPath): Promise<OutlineMenuState>
}

export type OutlineMenuState =
    | Readonly<{ type: "initial-menu" }>
    | OutlineMenuEvent
    | UpdateMenuBadgeEvent
    | ToggleMenuExpandEvent

const initialState: OutlineMenuState = { type: "initial-menu" }

export type OutlineMenuMaterial = Readonly<{
    infra: OutlineMenuInfra
    shell: OutlineMenuShell
    config: OutlineMenuConfig
}>
export type OutlineMenuInfra = Readonly<{
    loadMenuBadgeRemote: LoadMenuBadgeRemote
    ticketRepository: AuthTicketRepository
    menuExpandRepository: MenuExpandRepository
    menuExpandStore: MenuExpandStore
    menuBadgeStore: MenuBadgeStore
}>
export type OutlineMenuShell = Readonly<{
    detectTargetPath: MenuTargetPathDetecter
}>
export type OutlineMenuConfig = Readonly<{
    version: string
    menuTree: MenuTree
}>

export function initOutlineMenuAction(material: OutlineMenuMaterial): OutlineMenuAction {
    const { state, post } = initAtom({
        initialState,
        ignite: async (): Promise<OutlineMenuState> => {
            return outlineMenu(material, (event) => {
                const state = post(event)

                switch (event.type) {
                    case "succeed-to-load":
                        // 初期ロード完了で最初の badge 更新を行う
                        return updateBadge()

                    default:
                        return state
                }
            })
        },
    })

    return {
        state,

        updateBadge,
        show(path: MenuCategoryPath): Promise<OutlineMenuState> {
            return toggleMenuExpand(material, path, true, post)
        },
        hide(path: MenuCategoryPath): Promise<OutlineMenuState> {
            return toggleMenuExpand(material, path, false, post)
        },
    }

    function updateBadge(): Promise<OutlineMenuState> {
        return updateMenuBadge(material, post)
    }
}

type OutlineMenuEvent =
    | Readonly<{ type: "required-to-login" }>
    | Readonly<{ type: "succeed-to-load"; menu: Menu }>
    | Readonly<{ type: "repository-error"; err: RepositoryError }>

async function outlineMenu<S>(
    { infra, shell, config }: OutlineMenuMaterial,
    post: Post<OutlineMenuEvent, S>,
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
    { infra, shell, config }: OutlineMenuMaterial,
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
    { infra, shell, config }: OutlineMenuMaterial,
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
