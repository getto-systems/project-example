import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"
import { toResetTokenDestinationEmail } from "../../input/convert"

import { ModifyAuthUserAccountRemoteResult, ModifyAuthUserAccountRemote } from "../infra"

import { AuthUserAccountBasket } from "../../kernel/data"
import { ModifyAuthUserAccountFields } from "../data"
import { ResetTokenDestination } from "../../input/data"

export function newModifyAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): ModifyAuthUserAccountRemote {
    return (user, fields) => fetchRemote(feature, user, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: AuthUserAccountBasket,
    fields: ModifyAuthUserAccountFields,
): Promise<ModifyAuthUserAccountRemoteResult> {
    try {
        const mock = false
        if (mock) {
            return {
                success: true,
                value: {
                    loginId: user.loginId,
                    grantedRoles: fields.grantedRoles,
                    resetTokenDestination: fields.resetTokenDestination,
                },
            }
        }

        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "PATCH",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.modify.service.ModifyAuthUserAccountRequestPb,
                (message) => {
                    message.loginId = user.loginId
                    message.from = {
                        grantedRoles: Array.from(user.grantedRoles),
                        resetTokenDestination: user.resetTokenDestination,
                    }
                    message.to = {
                        grantedRoles: Array.from(fields.grantedRoles),
                        resetTokenDestination: fields.resetTokenDestination,
                    }
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.modify.service.ModifyAuthUserAccountResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return errorResponse(message.err)
        }
        if (!message.data) {
            return {
                success: false,
                err: { type: "infra-error", err: "data not exists in response" },
            }
        }
        return {
            success: true,
            value: responseData(user, message.data),
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function responseData(
    user: AuthUserAccountBasket,
    data: pb.auth.user.account.modify.service.IModifyAuthUserAccountDataPb,
): AuthUserAccountBasket {
    return {
        loginId: user.loginId,
        grantedRoles: data.grantedRoles || [],
        resetTokenDestination: resetTokenDestination(data.resetTokenDestination || {}),
    }

    function resetTokenDestination(
        destination: pb.auth.user.account.modify.service.IModifyResetTokenDestinationDataPb,
    ): ResetTokenDestination {
        switch (destination.type) {
            case "email":
                return toResetTokenDestinationEmail(destination.email || "")

            default:
                return { type: "none" }
        }
    }
}
function errorResponse(
    err: pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb,
): ModifyAuthUserAccountRemoteResult {
    switch (err) {
        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.CONFLICT:
            return { success: false, err: { type: "conflict" } }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb
            .INVALID_GRANTED_ROLE:
            return { success: false, err: { type: "invalid-granted-role" } }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb
            .INVALID_RESET_TOKEN_DESTINATION_EMAIL:
            return { success: false, err: { type: "invalid-reset-token-destination-email" } }
    }
}
