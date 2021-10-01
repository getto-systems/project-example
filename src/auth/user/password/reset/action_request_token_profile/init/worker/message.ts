import { WorkerProxySpec } from "../../../../../../../../ui/vendor/getto-application/action/worker/message"

import { RequestResetTokenEvent } from "../../../request_token/event"

import { ConvertBoardResult } from "../../../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "../../../request_token/data"

export type RequestResetTokenProfileProxyMaterial = Readonly<{
    requestToken: RequestTokenProfile["method"]
}>
export type RequestResetTokenProfileProxyMessage = RequestTokenProfile["message"]
export type RequestResetTokenProfileProxyResponse = RequestTokenProfile["response"]

type RequestTokenProfile = WorkerProxySpec<
    "requestToken",
    Readonly<{ fields: ConvertBoardResult<RequestResetTokenFields> }>,
    RequestResetTokenEvent
>
