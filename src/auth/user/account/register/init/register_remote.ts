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

import { RegisterAuthUserAccountRemoteResult, RegisterAuthUserAccountRemote } from "../infra"

import { RegisterAuthUserAccountRemoteError } from "../data"
import { AuthUserAccount } from "../../kernel/data"

export function newRegisterAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): RegisterAuthUserAccountRemote {
    return (fields) => fetchRemote(feature, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    fields: AuthUserAccount,
): Promise<RegisterAuthUserAccountRemoteResult> {
    const mock = false
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "POST",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.register.service.RegisterAuthUserAccountRequestPb,
                (message) => {
                    message.data = {
                        ...fields,
                        grantedRoles: Array.from(fields.grantedRoles),
                    }
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.register.service.RegisterAuthUserAccountResponsePb,
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
    err: pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb,
): RegisterAuthUserAccountRemoteError {
    switch (err) {
        case pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb
            .LOGIN_ID_ALREADY_REGISTERED:
            return { type: "login-id-already-registered" }

        case pb.auth.user.account.register.service.RegisterAuthUserAccountErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
