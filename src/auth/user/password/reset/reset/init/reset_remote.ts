import { env } from "../../../../../../y_environment/ui/env"
import pb from "../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../common/util/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../z_vendor/protobuf/helper"

import { convertCheckRemote } from "../../../../../ticket/authenticate/convert"

import { Clock } from "../../../../../../common/util/clock/infra"
import { ResetPasswordRemote, ResetPasswordRemoteResult } from "../infra"

import { ResetToken, ResetPasswordFields } from "../data"

export function newResetPasswordRemote(clock: Clock): ResetPasswordRemote {
    return (resetToken, fields) => fetchRemote(clock, resetToken, fields)
}

async function fetchRemote(
    clock: Clock,
    resetToken: ResetToken,
    fields: ResetPasswordFields,
): Promise<ResetPasswordRemoteResult> {
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
            path: "/auth/user/password/reset",
            method: "POST",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.password.reset.reset.service.ResetPasswordRequestPb,
                (message) => {
                    message.resetToken = resetToken
                    message.loginId = fields.loginId
                    message.newPassword = fields.newPassword
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.password.reset.reset.service.ResetPasswordMaskedResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return errorResponse(message.err)
        }
        return {
            success: true,
            value: convertCheckRemote(clock, message.granted?.permissions || []),
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.password.reset.reset.service.ResetPasswordErrorKindPb,
): ResetPasswordRemoteResult {
    switch (err) {
        case pb.auth.user.password.reset.reset.service.ResetPasswordErrorKindPb.INVALID_RESET:
            return { success: false, err: { type: "invalid-reset" } }

        case pb.auth.user.password.reset.reset.service.ResetPasswordErrorKindPb.ALREADY_RESET:
            return { success: false, err: { type: "already-reset" } }

        default:
            return { success: false, err: { type: "invalid-reset" } }
    }
}
