import { h } from "preact"

import { storyTemplate } from "../../../../../../ui/vendor/storybook/preact/story"

import { LoadSeasonFieldComponent } from "./load_season_field"

import { markSeason } from "../../load_season/test_helper"

import { mockLoadSeasonResource } from "../mock"

import { LoadSeasonCoreState } from "../core/action"

const options = ["success", "error"] as const

export default {
    title: "library/Example/Common/Example",
    argTypes: {
        load: {
            control: { type: "select", options },
        },
    },
}

type MockProps = Readonly<{
    load: typeof options[number]
    year: number
    err: string
}>
const template = storyTemplate<MockProps>((props) => {
    return h(LoadSeasonFieldComponent, { ...mockLoadSeasonResource(), state: state() })

    function state(): LoadSeasonCoreState {
        switch (props.load) {
            case "success":
                return { type: "succeed-to-load", value: markSeason({ year: props.year }) }

            case "error":
                return { type: "failed-to-load", err: { type: "infra-error", err: props.err } }
        }
    }
})

export const Example = template({ load: "success", year: new Date().getFullYear(), err: "" })
