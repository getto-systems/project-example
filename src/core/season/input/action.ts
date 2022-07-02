import {
    BoardValueFieldAction,
    initBoardValueFieldAction,
} from "../../../z_lib/ui/input/field/text"

import { seasonConverter, seasonToString } from "../kernel/convert"

import { DetectedSeason, Season, ValidateSeasonError } from "../kernel/data"

export type SeasonFieldAction = BoardValueFieldAction<DetectedSeason, ValidateSeasonError>

export function initSeasonFieldAction(availableSeasons: readonly Season[]): SeasonFieldAction {
    return initBoardValueFieldAction({
        map: (value) => {
            if (value.default) {
                return ""
            } else {
                return seasonToString(value.season)
            }
        },
        convert: (value) => seasonConverter(availableSeasons, value),
    })
}
