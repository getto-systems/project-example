import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { SearchAuthUserAccountRemote } from "../infra"
import { ticker } from "../../../../../z_lib/ui/timer/helper"

export function newSearchAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): SearchAuthUserAccountRemote {
    return async (fields) => {
        try {
            const mock = false
            if (mock) {
                await ticker({ wait_millisecond: 3000 }, () => null)
                return {
                    success: true,
                    value: {
                        page: { offset: 0, limit: 10, all: 25 },
                        summary: {},
                        users: [{ loginID: "admin", grantedRoles: ["dev-docs"] }],
                    },
                }
            }

            const body = encodeProtobuf(
                pb.auth.user.account.api.SearchAuthUserAccountApiRequestPb,
                (message) => {
                    message.offset = parseInt(fields.offset)
                    message.sortKey = fields.sort.key
                    message.sortOrder = fields.sort.order
                    message.loginId = fields.loginID
                },
            )

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: `/auth/user/account/search/${body}`,
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(
                pb.auth.user.account.api.SearchAuthUserAccountApiResponsePb,
                await response.text(),
            )
            return {
                success: true,
                value: {
                    page: { offset: result.offset, limit: result.limit, all: result.all },
                    summary: {},
                    users: result.users.map((user) => ({
                        loginID: user.loginId || "",
                        grantedRoles: user.grantedRoles || [],
                    })),
                },
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
