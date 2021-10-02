import {
    ChangePasswordProxyMessage,
    ChangePasswordProxyResponse,
} from "../../../../../../src/auth/user/password/action_change/init/worker/message"
import {
    RequestResetTokenProfileProxyMessage,
    RequestResetTokenProfileProxyResponse,
} from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/message"

export type ProfileForegroundMessage =
    | Readonly<{ type: "password-change"; message: ChangePasswordProxyMessage }>
    | Readonly<{
          type: "password-reset-requestToken"
          message: RequestResetTokenProfileProxyMessage
      }>

export type ProfileBackgroundMessage =
    | Readonly<{ type: "password-change"; response: ChangePasswordProxyResponse }>
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestResetTokenProfileProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
