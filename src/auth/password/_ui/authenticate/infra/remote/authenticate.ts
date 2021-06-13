import { env } from "../../../../../../y_environment/_ui/env"
import {
    AuthenticatePasswordResult_pb,
    AuthenticatePassword_pb,
} from "../../../../../_ui/y_protobuf/api_pb.js"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../../ui/vendor/getto-application/infra/remote/helper"
import {
    apiInfraError,
    apiRequest,
    apiStatusError,
} from "../../../../../../z_details/_ui/api/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../ui/vendor/getto-application/infra/remote/feature"

import { AuthenticatePasswordRemotePod } from "../../infra"

import {
    ApiAuthenticateResponse,
    ApiCommonError,
    ApiResult,
} from "../../../../../../z_details/_ui/api/data"

export function newAuthenticatePasswordRemote(
    feature: RemoteOutsideFeature,
): AuthenticatePasswordRemotePod {
    type AuthenticateResult = ApiResult<ApiAuthenticateResponse, ApiCommonError | AuthenticateError>
    type AuthenticateError = Readonly<{ type: "invalid-password" }>

    return convertRemote(async (fields): Promise<AuthenticateResult> => {
        try {
            const mock = false
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: { roles: ["admin", "dev-docs"] } }
            }

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/auth/password/authenticate",
                "POST",
            )
            const response = await fetch(request.url, {
                ...request.options,
                body: encodeProtobuf(AuthenticatePassword_pb, (message) => {
                    message.loginId = fields.loginID
                    message.password = fields.password
                }),
            })

            if (!response.ok) {
                return apiStatusError(response.status)
            }

            const result = decodeProtobuf(AuthenticatePasswordResult_pb, await response.text())
            if (!result.success) {
                return { success: false, err: { type: "invalid-password" } }
            }
            return {
                success: true,
                value: {
                    roles: result.value?.roles || [],
                },
            }
        } catch (err) {
            return apiInfraError(err)
        }
    })
}
