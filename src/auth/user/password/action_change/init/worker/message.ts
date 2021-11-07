import { WorkerProxySpec } from "../../../../../../../ui/vendor/getto-application/action/worker/message"

import { ChangePasswordEvent } from "../../../change/event"

import { ConvertBoardResult } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"
import { ChangePasswordFields } from "../../../change/data"

export type ChangePasswordProxyMaterial = Readonly<{
    change: Change["method"]
}>
export type ChangePasswordProxyMessage = Change["message"]
export type ChangePasswordProxyResponse = Change["response"]

type Change = WorkerProxySpec<
    "change",
    Readonly<{ fields: ConvertBoardResult<ChangePasswordFields> }>,
    ChangePasswordEvent
>
