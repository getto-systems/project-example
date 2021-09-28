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
import { lniClass, lnir } from "../../../../z_lib/ui/icon/line_icon"

import { LoadMenuComponent } from "./load_menu"

import { mockLoadMenuAction, mockMenu } from "../mock"

import { LoadMenuState } from "../action"

import { Menu } from "../../kernel/data"

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
    label: string
    badgeCount: number
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
            menu: mockLoadMenuAction(menu()),
            state: state(),
        }),
    })

    function state(): LoadMenuState {
        switch (props.load) {
            case "success":
                return { type: "succeed-to-load", menu: menu() }

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
                    menu: menu(),
                    err: { type: props.load, err: props.err },
                }
        }
    }

    function menu(): Menu {
        return mockMenu(props.label, lniClass(lnir("home")), props.badgeCount)
    }
})

export const LoadMenu = template({ load: "success", label: "ホーム", badgeCount: 99, err: "" })
