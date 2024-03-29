import { env } from "../../../../../../../y_environment/ui/env"
import pb from "../../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../common/util/protobuf/helper"

import {
    ChangeResetTokenDestinationRemoteResult,
    ChangeResetTokenDestinationRemote,
} from "../infra"

import { ResetTokenDestination } from "../../kernel/data"
import { LoginId } from "../../../../../login_id/kernel/data"
import { ChangeResetTokenDestinationRemoteError } from "../data"

export function newModifyAuthUserAccountRemote(): ChangeResetTokenDestinationRemote {
    return (user, fields) => fetchRemote(user, fields)
}

async function fetchRemote(
    user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
    fields: ResetTokenDestination,
): Promise<ChangeResetTokenDestinationRemoteResult> {
    const mock = false
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/password/reset/token-destination",
            method: "PATCH",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.password.reset.token_destination.change.service
                    .ChangeResetTokenDestinationRequestPb,
                (message) => {
                    message.loginId = user.loginId
                    message.from = user.resetTokenDestination
                    message.to = fields
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.password.reset.token_destination.change.service
                .ChangeResetTokenDestinationResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return { success: false, err: errorResponse(message.err) }
        }
        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.password.reset.token_destination.change.service.ChangeResetTokenDestinationErrorKindPb,
): ChangeResetTokenDestinationRemoteError {
    switch (err) {
        case pb.auth.user.password.reset.token_destination.change.service
            .ChangeResetTokenDestinationErrorKindPb.CONFLICT:
            return { type: "conflict" }

        case pb.auth.user.password.reset.token_destination.change.service
            .ChangeResetTokenDestinationErrorKindPb.NOT_FOUND:
            return { type: "not-found" }

        case pb.auth.user.password.reset.token_destination.change.service
            .ChangeResetTokenDestinationErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
