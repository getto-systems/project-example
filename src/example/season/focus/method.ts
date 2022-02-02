import { FocusSeasonInfra } from "./infra"

import { FocusSeasonEvent } from "./event"

import { BoardValue } from "../../../../ui/vendor/getto-application/board/kernel/data"
import { seasonBoardConverter } from "../kernel/convert"

export interface FocusSeasonMethod {
    <S>(season: BoardValue, post: Post<FocusSeasonEvent, S>): Promise<S>
}

interface Focus {
    (infra: FocusSeasonInfra): FocusSeasonMethod
}
export const focusSeason: Focus = (infra) => async (value, post) => {
    const { clock, season, config } = infra

    const convertResult = seasonBoardConverter(value)
    if (!convertResult.valid) {
        return post({ type: "invalid-season" })
    }

    if (convertResult.default) {
        const result = await season.remove()
        if (!result.success) {
            return post({ type: "failed-to-focus", err: result.err })
        }
        return post({ type: "succeed-to-focus" })
    }

    const result = await season.set({
        season: convertResult.season,
        expires: clock.now().getTime() + config.focusSeasonExpire.expire_millisecond,
    })
    if (!result.success) {
        return post({ type: "failed-to-focus", err: result.err })
    }

    return post({ type: "succeed-to-focus" })
}

interface Post<E, S> {
    (event: E): S
}
