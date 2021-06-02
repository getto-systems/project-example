// この infra は UI で実行時に構築されるので、例外的に action で参照していい
import { BoardValueStore } from "../../input/infra"

import { ApplicationAction } from "../../../action/action"

import { BoardValue } from "../../kernel/data"

export interface InputBoardValueAction extends ApplicationAction {
    readonly storeLinker: BoardValueStoreLinker

    subscribeInputEvent(handler: InputBoardValueHandler): void
    triggerInputEvent(): void

    get(): BoardValue
    set(value: BoardValue): void
    clear(): void
}
export interface BoardValueStoreLinker {
    link(store: BoardValueStore): void
    unlink(): void
}
export interface InputBoardValueHandler {
    (): void
}
