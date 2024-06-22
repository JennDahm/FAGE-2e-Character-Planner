
use dioxus::prelude::*;

use fage2e;

#[component]
pub fn AncestryBenefitSelections<B: fage2e::AncestryBenefit + 'static>(
    mut benefits: Signal<fage2e::AncestryBenefitSelections<B>>
) -> Element {
    let mut selections: Signal<Vec<B>> = use_signal(move || {
        let benefits = benefits();
        benefits.selection1.into_iter().chain(benefits.selection2.into_iter()).collect()
    });
    let selections_rolled = use_memo(move || {
        benefits().selections_were_rolled
    });
    let options = use_signal(|| B::iter().collect());
    let disabled_options = use_memo(move || {
        if selections().len() == 0 || selections_rolled() {
            return Vec::new();
        }
        else {
            return B::iter().filter(|b| b.counts_as_two()).collect();
        }
    });
    let max_selections = use_memo(move || {
        let selections = selections();
        if !selections_rolled() && selections.len() > 0 && selections[0].counts_as_two() {
            return 1;
        }
        else {
            return 2;
        }
    });

    use_effect(move || {
        let selections = selections();
        if selections.len() == 0 {
            (*benefits.write()).selection1 = None;
            (*benefits.write()).selection2 = None;
        } else if selections.len() == 1 {
            (*benefits.write()).selection1 = Some(selections[0]);
            (*benefits.write()).selection2 = None;
        } else {
            (*benefits.write()).selection1 = Some(selections[0]);
            (*benefits.write()).selection2 = Some(selections[1]);
        }
    });

    use crate::widget::Button;
    use crate::widget::DisableableMultiSelector;

    rsx! {
        Button {
            text: "Roll",
            disabled: false,
            onclick: move |_| {
                (*benefits.write()) = fage2e::AncestryBenefitSelections::<B>::roll();
                let benefits = benefits();
                (*selections.write()) = benefits.selection1.into_iter().chain(benefits.selection2.into_iter()).collect();
            },
        }
        br {}
        input {
            r#type: "checkbox",
            name: "benefits_rolled",
            onchange: move |event| {
                (*benefits.write()).selections_were_rolled = event.checked();
            },
            checked: selections_rolled(),
        }
        label {
            r#for: "benefits_rolled",
            "Benefits were rolled"
        }
        DisableableMultiSelector { options, disabled_options, selections, max_selections }
    }
}
