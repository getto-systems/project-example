import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { readSearchAuthUserAccountSortKey } from "../convert"
import { parseSearchSort } from "../../../../../common/util/search/sort/convert"
import { parseSearchPage } from "../../../../../common/util/search/kernel/convert"
import { toGranted } from "../../input/granted/convert"
import { restoreLoginId } from "../../../login_id/input/convert"
import { restoreAuthUserField } from "../../kernel/convert"

import { SearchAuthUserAccountRemote, SearchAuthUserAccountRemoteResult } from "../infra"

import { defaultSearchAuthUserAccountSort, SearchAuthUserAccountFilter } from "../data"
import { AuthUserAccount } from "../../kernel/data"
import { restoreResetTokenDestination } from "../../../password/reset/token_destination/kernel/convert"

export function newSearchAuthUserAccountRemote(): SearchAuthUserAccountRemote {
    return (filter) => fetchRemote(filter)
}
async function fetchRemote(
    filter: SearchAuthUserAccountFilter,
): Promise<SearchAuthUserAccountRemoteResult> {
    try {
        const mock = false
        if (mock) {
            //await ticker({ wait_millisecond: 3000 }, () => null)
            const list: AuthUserAccount[] = []
            for (let i = 0; i < 50; i++) {
                list.push({
                    loginId: restoreLoginId(`user-${i}`),
                    granted: [],
                    resetTokenDestination: { type: "none" },
                    memo: restoreAuthUserField(`no. ${i}`),
                })
            }
            return {
                success: true,
                value: {
                    page: { offset: 0, limit: 1000, count: list.length },
                    sort: { key: defaultSearchAuthUserAccountSort, order: "normal" },
                    list,
                },
            }
        }

        const body = encodeProtobuf(
            pb.auth.user.account.search.service.SearchAuthUserAccountRequestPb,
            (message) => {
                message.offset = parseInt(filter.offset)
                message.sort = filter.sort
                if (filter.loginId.filter) {
                    message.loginId = [filter.loginId.value]
                }
                message.granted = Array.from(filter.granted)
            },
        )

        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: `/auth/user/account/search/${body}`,
            method: "GET",
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
                page: parseSearchPage(message.page),
                sort: parseSearchSort(
                    message.sort,
                    defaultSearchAuthUserAccountSort,
                    readSearchAuthUserAccountSortKey,
                ),
                list: message.users.map(
                    (user): AuthUserAccount => ({
                        loginId: restoreLoginId(user.loginId || ""),
                        granted: toGranted(user.granted || []),
                        resetTokenDestination: restoreResetTokenDestination({
                            type: user.resetTokenDestination?.type || "",
                            email: user.resetTokenDestination?.email || "",
                        }),
                        memo: restoreAuthUserField(user.memo || ""),
                    }),
                ),
            },
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}
