import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { Clock } from "../../../../../common/util/clock/infra"
import { AuthenticatePasswordRemote } from "../infra"

import { convertCheckRemote } from "../../../../ticket/authenticate/convert"

export function newAuthenticatePasswordRemote(clock: Clock): AuthenticatePasswordRemote {
    return async (fields) => {
        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: convertCheckRemote(clock, []),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/user/password/authenticate",
                method: "POST",
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.auth.user.password.authenticate.service.AuthenticateWithPasswordRequestPb,
                    (message) => {
                        message.loginId = fields.loginId
                        message.password = fields.password
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.user.password.authenticate.service.AuthenticateWithPasswordMaskedResponsePb,
                await response.text(),
            )
            if (!message.success) {
                return { success: false, err: { type: "invalid-password" } }
            }
            return {
                success: true,
                value: convertCheckRemote(clock, message.granted?.permissions || []),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
