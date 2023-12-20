import {
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "../../../../../auth/user/password/change/detail/worker/message"
import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../../auth/user/password/reset/request_token/detail/worker/message"

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
