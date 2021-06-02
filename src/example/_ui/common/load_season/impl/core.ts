import { defaultSeason, seasonRepositoryConverter } from "./converter"

import { LoadSeasonInfra } from "../infra"

import { LoadSeasonMethod } from "../method"

interface Load {
    (infra: LoadSeasonInfra): LoadSeasonMethod
}
export const loadSeason: Load = (infra) => async (post) => {
    const { clock } = infra
    const season = infra.season(seasonRepositoryConverter)

    const result = await season.get()
    if (!result.success) {
        return post({ type: "failed-to-load", err: result.err })
    }
    if (!result.found) {
        return post({ type: "succeed-to-load", value: defaultSeason(clock) })
    }
    return post({ type: "succeed-to-load", value: result.value })
}
