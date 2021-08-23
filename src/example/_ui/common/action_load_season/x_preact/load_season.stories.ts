import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { copyright, siteInfo } from "../../../../site"
import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../../../ui/vendor/getto-css/preact/layout/app"

import { LoadMenuEntry } from "../../../../outline/_ui/action_load_menu/x_preact/load_menu"
import { LoadSeasonEntry } from "./load_season"

import { mockLoadMenuAction, mockMenu_home } from "../../../../outline/_ui/action_load_menu/mock"

import { mockLoadSeasonResource } from "../mock"

export default {
    title: "library/Example/Common/Load Season",
    parameters: {
        layout: "fullscreen",
    },
}

type MockProps = {
    // no props
}
const template = storyTemplate<MockProps>(() => {
    return appLayout({
        siteInfo,
        header: [h(LoadSeasonEntry, mockLoadSeasonResource())],
        main: appMain({
            header: mainHeader([mainTitle("タイトル")]),
            body: mainBody("コンテンツ"),
            copyright,
        }),
        menu: h(LoadMenuEntry, { menu: mockLoadMenuAction(mockMenu_home()) }),
    })
})

export const LoadSeason = template({})
