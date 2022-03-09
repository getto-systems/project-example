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

import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "../infra"

import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountFilter } from "../data"
import { readSearchAuthUserAccountSortKey } from "../convert"
import { parseSearchSort } from "../../../../../z_lib/ui/search/sort/convert"
import { AuthUserAccountBasket } from "../../kernel/data"

export function newSearchAuthUserAccountRemote(
    feature: RemoteOutsideFeature,
): SearchAuthUserAccountRemote {
    return (filter) => fetchRemote(feature, filter)
}
async function fetchRemote(
    feature: RemoteOutsideFeature,
    filter: SearchAuthUserAccountFilter,
): Promise<SearchAuthUserAccountRemoteResult> {
    try {
        const mock = false
        if (mock) {
            //await ticker({ wait_millisecond: 3000 }, () => null)
            const users: AuthUserAccountBasket[] = []
            for (let i = 0; i < 50; i++) {
                users.push({
                    loginID: `user-${i}`,
                    grantedRoles: [],
                })
            }
            return {
                success: true,
                value: {
                    page: { offset: 0, limit: 1000, all: users.length },
                    sort: { key: defaultSearchAuthUserAccountSort, order: "normal" },
                    users,
                },
            }
        }

        const body = encodeProtobuf(
            pb.auth.user.account.search.service.SearchAuthUserAccountRequestPb,
            (message) => {
                message.offset = parseInt(filter.offset)
                message.sortKey = filter.sort.key
                message.sortOrder = filter.sort.order
                if (filter.loginID.search) {
                    message.loginId = [filter.loginID.value]
                }
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

        const message = decodeProtobuf(
            pb.auth.user.account.search.service.SearchAuthUserAccountResponsePb,
            await response.text(),
        )
        return {
            success: true,
            value: {
                page: { offset: message.offset, limit: message.limit, all: message.all },
                sort: parseSearchSort(
                    {
                        key: message.sortKey,
                        order: message.sortOrder,
                    },
                    defaultSearchAuthUserAccountSort,
                    readSearchAuthUserAccountSortKey,
                ),
                users: message.users.map((user) => ({
                    loginID: user.loginId || "",
                    grantedRoles: user.grantedRoles || [],
                })),
            },
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}
