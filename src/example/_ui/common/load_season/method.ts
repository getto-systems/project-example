import { LoadSeasonInfra } from "./infra"

import { LoadSeasonEvent } from "./event"

import { defaultSeason } from "./converter"

export interface LoadSeasonMethod {
    <S>(post: Post<LoadSeasonEvent, S>): Promise<S>
}

interface Load {
    (infra: LoadSeasonInfra): LoadSeasonMethod
}
export const loadSeason: Load = (infra) => async (post) => {
    const { clock, season } = infra

    const result = await season.get()
    if (!result.success) {
        return post({ type: "failed-to-load", err: result.err })
    }
    if (!result.found) {
        return post({ type: "succeed-to-load", value: defaultSeason(clock) })
    }
    return post({ type: "succeed-to-load", value: result.value })
}

interface Post<E, S> {
    (event: E): S
}
