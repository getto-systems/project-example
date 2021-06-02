import { AuthenticatePasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { AuthenticatePasswordFields } from "./data"

export interface AuthenticatePasswordMethod {
    <S>(
        fields: ConvertBoardResult<AuthenticatePasswordFields>,
        post: Post<AuthenticatePasswordEvent, S>,
    ): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
