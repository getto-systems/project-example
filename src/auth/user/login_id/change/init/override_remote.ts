import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { ChangePasswordRemoteResult, OverrideLoginIdRemote } from "../infra"

import { OverrideLoginIdFields } from "../data"
import { LoginId } from "../../input/data"

export function newOverrideLoginIdRemote(feature: RemoteOutsideFeature): OverrideLoginIdRemote {
    return (user, fields) => fetchRemote(feature, user, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: Readonly<{ loginId: LoginId }>,
    fields: OverrideLoginIdFields,
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
            path: "/auth/user/login-id/override",
            method: "PATCH",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.loginId.change.service.OverrideLoginIdRequestPb,
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
            pb.auth.user.loginId.change.service.OverrideLoginIdResponsePb,
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
    err: pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb,
): ChangePasswordRemoteResult {
    switch (err) {
        case pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb.NOT_FOUND:
            return { success: false, err: { type: "not-found" } }

        case pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb.INVALID:
            return { success: false, err: { type: "invalid" } }

        case pb.auth.user.loginId.change.service.OverrideLoginIdErrorKindPb.ALREADY_REGISTERED:
            return { success: false, err: { type: "already-registered" } }
    }
}
