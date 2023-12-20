import { h } from "preact"
import { PreactNode } from "../../../../../common/x_preact/vnode"

import { remoteCommonErrorReason } from "../../../../../common/util/remote/x_error/reason"
import { SearchPager } from "../../../../../common/util/search/offset/x_preact/offset"

import { SearchAuthUserAccountAction } from "../action"

import { RemoteCommonError } from "../../../../../common/util/remote/data"

type Props = Readonly<{
    search: SearchAuthUserAccountAction
}>
export function SearchAuthUserAccountPager(props: Props): PreactNode {
    return h(SearchPager<RemoteCommonError>, {
        search: props.search,
        error: (err) => {
            return remoteCommonErrorReason(err, (reason) => [
                `${reason.message}により検索に失敗しました`,
                ...reason.detail,
            ])
        },
    })
}
