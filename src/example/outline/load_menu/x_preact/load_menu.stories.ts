import { h } from "preact"

import { storyTemplate } from "../../../../../ui/vendor/storybook/preact/story"

import {
    appLayout,
    appMain,
    mainHeader,
    mainTitle,
    mainBody,
} from "../../../../../ui/vendor/getto-css/preact/layout/app"

import { copyright, siteInfo } from "../../../site"

import { LoadMenuComponent } from "./load_menu"

import { mockLoadMenuShell } from "../init/mock"
import { mockRemoteInfraError } from "../../../../z_lib/ui/remote/mock"

import { initMemoryDB } from "../../../../z_lib/ui/repository/init/memory"
import { initMenuBadgeStore, initMenuExpandStore } from "../init/store"

import { initLoadMenuAction, LoadMenuState } from "../action"

const options = [
    "success",
    "required-to-login",
    "repository-error",
    "server-error",
    "infra-error",
] as const

export default {
    title: "library/Outline/Menu/Load Menu",
    parameters: {
        layout: "fullscreen",
    },
    argTypes: {
        load: {
            control: { type: "select", options },
        },
    },
}

type MockProps = Readonly<{
    load: typeof options[number]
    err: string
}>
const template = storyTemplate<MockProps>((props) => {
    return appLayout({
        siteInfo,
        header: [],
        main: appMain({
            header: mainHeader([mainTitle("タイトル")]),
            body: mainBody("コンテンツ"),
            copyright,
        }),
        menu: h(LoadMenuComponent, {
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
            state: state(),
        }),
    })

    function state(): LoadMenuState {
        switch (props.load) {
            case "success":
                return { type: "succeed-to-load", menu: [] }

            case "required-to-login":
                return { type: props.load }

            case "repository-error":
                return {
                    type: "repository-error",
                    err: { type: "infra-error", err: props.err },
                }

            default:
                return {
                    type: "failed-to-update",
                    menu: [],
                    err: { type: props.load, err: props.err },
                }
        }
    }
})

export const LoadMenu = template({ load: "success", err: "" })
