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

import { ModifyAuthUserAccountRemoteResult, ModifyAuthUserAccountRemote } from "../infra"

import { ModifyAuthUserAccountFields, ModifyAuthUserAccountRemoteError } from "../data"
import { LoginId } from "../../../login_id/kernel/data"
import { GrantedAuthRole } from "../../../kernel/data"

export function newModifyAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): ModifyAuthUserAccountRemote {
    return (user, fields) => fetchRemote(feature, user, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: Readonly<{ loginId: LoginId; grantedRoles: readonly GrantedAuthRole[] }>,
    fields: ModifyAuthUserAccountFields,
): Promise<ModifyAuthUserAccountRemoteResult> {
    const mock = true
    if (mock) {
        return { success: true, value: true }
    }

    try {
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
                    }
                    message.to = {
                        grantedRoles: Array.from(fields.grantedRoles),
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
            return { success: false, err: errorResponse(message.err) }
        }
        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb,
): ModifyAuthUserAccountRemoteError {
    switch (err) {
        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.CONFLICT:
            return { type: "conflict" }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.NOT_FOUND:
            return { type: "not-found" }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
