import { FindNextVersionAction, FindNextVersionState } from "../action"

import { applicationPath } from "../helper"

import { ConvertLocationResult } from "../../../../z_lib/ui/location/data"
import { ApplicationTargetPath } from "../data"

type Props = Readonly<{
    findNext: FindNextVersionAction
}>
export function MoveToNextVersion(props: Props): void {
    // /${version}/index.html とかで実行する
    try {
        props.findNext.subscriber.subscribe(handleState)
    } catch (err) {
        handleError(err)
    }

    function handleState(state: FindNextVersionState): true {
        switch (state.type) {
            case "initial":
            case "take-longtime":
                // work in progress...
                return true

            case "success":
                redirect(state.upToDate, state.version, state.target)
                return true

            case "failed":
                handleError(state.err)
                return true
        }
    }
    function redirect(
        upToDate: boolean,
        version: string,
        target: ConvertLocationResult<ApplicationTargetPath>,
    ) {
        const path = redirectPath(upToDate, version, target)
        if (path.redirect) {
            location.href = path.path
        }

        props.findNext.subscriber.unsubscribe(handleState)
    }
    function redirectPath(
        upToDate: boolean,
        version: string,
        target: ConvertLocationResult<ApplicationTargetPath>,
    ): RedirectPath {
        // application target が指定されたらリダイレクトする
        if (target.valid) {
            if (target.value.specified) {
                return { redirect: true, path: applicationPath(version, target) }
            }
        }

        // 次のバージョンが見つかったらリダイレクトする
        if (!upToDate) {
            return { redirect: true, path: applicationPath(version, target) }
        }

        // 今のバージョンが最新ならリダイレクトしない
        return { redirect: false }
    }

    function handleError(err: unknown) {
        // エラーはどうしようもないので console.log でお茶を濁す
        console.log(err)
        props.findNext.subscriber.unsubscribe(handleState)
    }
}

type RedirectPath = Readonly<{ redirect: false }> | Readonly<{ redirect: true; path: string }>
