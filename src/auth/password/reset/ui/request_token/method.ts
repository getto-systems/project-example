import { RequestResetTokenEvent } from "./event"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "./data"

export interface RequestResetTokenMethod {
    <S>(
        fields: ConvertBoardResult<RequestResetTokenFields>,
        post: Post<RequestResetTokenEvent, S>,
    ): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
