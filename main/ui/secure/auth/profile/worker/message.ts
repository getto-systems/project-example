import {
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "../../../../../../src/auth/user/password/change/init/worker/message"
import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../../../src/auth/user/password/reset/request_token/init/worker/message"

export type ProfileForegroundMessage =
    | Readonly<{ type: "password-change"; message: ChangePasswordProxyMessage }>
    | Readonly<{
          type: "password-reset-requestToken"
          message: RequestResetTokenProxyMessage
      }>

export type ProfileBackgroundMessage =
    | Readonly<{ type: "password-change"; response: ChangePasswordProxyResponse }>
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestResetTokenProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
