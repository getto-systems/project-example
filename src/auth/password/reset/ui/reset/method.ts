import { ResetPasswordEvent } from "./event"

import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { ResetToken } from "../data"
import { ResetPasswordFields } from "./data"
import { LocationTypes } from "../../../../../../ui/vendor/getto-application/location/infra"

export interface ResetPasswordPod {
    (detecter: ResetPasswordLocationDetecter): ResetPasswordMethod
}

type ResetLocationTypes = LocationTypes<ResetToken>
export type ResetPasswordLocationDetecter = ResetLocationTypes["detecter"]
export type ResetPasswordLocationDetectMethod = ResetLocationTypes["method"]
export type ResetPasswordLocationInfo = ResetLocationTypes["info"]

export interface ResetPasswordMethod {
    <S>(
        fields: ConvertBoardResult<ResetPasswordFields>,
        post: Post<ResetPasswordEvent, S>,
    ): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
