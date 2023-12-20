import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../common/util/protobuf/helper"

import { ChangePasswordRemoteResult, OverwriteLoginIdRemote } from "../infra"

import { OverwriteLoginIdFields } from "../data"
import { LoginId } from "../../kernel/data"

export function newOverwriteLoginIdRemote(): OverwriteLoginIdRemote {
    return (user, fields) => fetchRemote(user, fields)
}

async function fetchRemote(
    user: Readonly<{ loginId: LoginId }>,
    fields: OverwriteLoginIdFields,
): Promise<ChangePasswordRemoteResult> {
    try {
        const mock = false
        if (mock) {
            return {
                success: true,
                value: true,
            }
        }

        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/login-id/overwrite",
            method: "PATCH",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.loginId.change.service.OverwriteLoginIdRequestPb,
                (message) => {
                    message.loginId = user.loginId
                    message.newLoginId = fields.newLoginId
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.loginId.change.service.OverwriteLoginIdResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return errorResponse(message.err)
        }
        return {
            success: true,
            value: true,
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.loginId.change.service.OverwriteLoginIdErrorKindPb,
): ChangePasswordRemoteResult {
    switch (err) {
        case pb.auth.user.loginId.change.service.OverwriteLoginIdErrorKindPb.NOT_FOUND:
            return { success: false, err: { type: "not-found" } }

        case pb.auth.user.loginId.change.service.OverwriteLoginIdErrorKindPb.INVALID:
            return { success: false, err: { type: "invalid" } }

        case pb.auth.user.loginId.change.service.OverwriteLoginIdErrorKindPb.ALREADY_REGISTERED:
            return { success: false, err: { type: "already-registered" } }
    }
}
