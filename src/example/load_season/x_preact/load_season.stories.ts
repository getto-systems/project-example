import { h } from "preact"

import { storyTemplate } from "../../../../ui/vendor/storybook/preact/story"

import { copyright, siteInfo } from "../../site"
import {
    appLayout,
    appMain,
    mainBody,
    mainHeader,
    mainTitle,
} from "../../../../ui/vendor/getto-css/preact/layout/app"

import { LoadMenuEntry } from "../../outline/load_menu/x_preact/load_menu"
import { LoadSeasonEntry } from "./load_season"

import { initMemoryDB } from "../../../z_lib/ui/repository/init/memory"
import { newClock } from "../../../z_lib/ui/clock/init"
import { initLoadMenuAction } from "../../outline/load_menu/action"
import { initMenuBadgeStore, initMenuExpandStore } from "../../outline/load_menu/init/store"

import { mockRemoteInfraError } from "../../../z_lib/ui/remote/mock"
import { mockLoadMenuShell } from "../../outline/load_menu/init/mock"

import { initLoadSeasonAction } from "../action"

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
        header: [
            h(LoadSeasonEntry, {
                season: initLoadSeasonAction({
                    seasonRepository: initMemoryDB(),
                    clock: newClock(),
                }),
            }),
        ],
        main: appMain({
            header: mainHeader([mainTitle("タイトル")]),
            body: mainBody("コンテンツ"),
            copyright,
        }),
        menu: h(LoadMenuEntry, {
            menu: initLoadMenuAction(
                {
                    version: "0.0.0",
                    menuTree: [],
                    getMenuBadgeRemote: async () => mockRemoteInfraError,
                    ticketRepository: initMemoryDB(),
                    menuExpandRepository: initMemoryDB(),
                    menuExpandStore: initMenuExpandStore(),
                    menuBadgeStore: initMenuBadgeStore(),
                },
                mockLoadMenuShell(new URL("https://example.com"), "0.0.0"),
            ),
        }),
    })
})

export const LoadSeason = template({})
