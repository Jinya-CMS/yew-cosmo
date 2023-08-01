use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTableProps {
    pub headers: Vec<AttrValue>,
    #[prop_or_default]
    pub children: ChildrenWithProps<CosmoTableRow>,
}

#[styled_component(CosmoTable)]
pub fn table(props: &CosmoTableProps) -> Html {
    let table_style = use_style!(r#"
border-collapse: collapse;

thead tr {
    border-bottom: 2px solid var(--primary-color);
}

thead tr th {
    font-weight: var(--font-weight-bold);
    padding: 0 8px 4px;
    text-transform: capitalize;
}

tbody tr td {
    border-bottom: 1px solid var(--control-border-color);
    font-weight: var(--font-weight-light);
    padding: 2px 8px;
}

tr:nth-child(2n-1) td {
    background: var(--table-stripe-color);
}
    "#);

    html!(
        <table class={table_style}>
            <thead>
                <tr>
                    {for props.headers.iter().map(|header| html!(<th>{header}</th>))}
                </tr>
            </thead>
            <tbody>
                {for props.children.iter()}
            </tbody>
        </table>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTableRowProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<CosmoTableCell>,
}

#[styled_component(CosmoTableRow)]
pub fn table_row(props: &CosmoTableRowProps) -> Html {
    html!(
        <tr>
            {for props.children.iter()}
        </tr>
    )
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTableCellProps {
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(CosmoTableCell)]
pub fn table_cell(props: &CosmoTableCellProps) -> Html {
    html!(
        <td>
            {for props.children.iter()}
        </td>
    )
}
