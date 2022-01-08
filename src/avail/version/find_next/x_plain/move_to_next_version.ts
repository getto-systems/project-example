import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"
import { FindNextVersionAction, FindNextVersionState } from "../action"

import { applicationPath } from "../helper"

import { ConvertLocationResult } from "../../../../z_lib/ui/location/data"
import { ApplicationTargetPath } from "../data"

export function MoveToNextVersionEntry(view: ApplicationView<FindNextVersionAction>): void {
    // /${version}/index.html とかで実行する
    const findNext = view.resource
    try {
        findNext.subscriber.subscribe(handleState)
        findNext.ignite()
    } catch (err) {
        handleError(err)
    }

    function handleState(state: FindNextVersionState) {
        switch (state.type) {
            case "initial-next-version":
            case "take-longtime-to-find":
                // work in progress...
                return

            case "succeed-to-find":
                redirect(state.upToDate, state.version, state.target)
                return

            case "failed-to-find":
                handleError(state.err)
                return

            default:
                assertNever(state)
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

        view.terminate()
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
        view.terminate()
    }
}

type RedirectPath = Readonly<{ redirect: false }> | Readonly<{ redirect: true; path: string }>

function assertNever(_: never): never {
    throw new Error("NEVER")
}
