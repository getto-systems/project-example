import { InputBoardValueAction } from "./core/action"

import { BoardValueStoreConnector, InputBoardEventPublisher } from "../input/infra"

import { InputBoardValueType } from "../input/data"

export type InputBoardValueResource = Readonly<{
    type: InputBoardValueType
    input: InputBoardValueAction
}>

export interface InputBoardAction {
    // 例外的に infra をそのまま公開する
    // input を infra として使用するので、この要素は action よりも infra に近い
    readonly connector: BoardValueStoreConnector
    readonly publisher: InputBoardEventPublisher
}
