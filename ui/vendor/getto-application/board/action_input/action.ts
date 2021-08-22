import { BoardValueStoreConnector, InputBoardEventPublisher } from "../input/infra"

export interface InputBoardAction {
    // 例外的に infra をそのまま公開する
    // input を infra として使用するので、この要素は action よりも infra に近い
    readonly connector: BoardValueStoreConnector
    readonly publisher: InputBoardEventPublisher
}
