use stylist::yew::{styled_component, use_style};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{Key, VChild};

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTableProps {
    pub headers: Vec<AttrValue>,
    #[prop_or_default]
    pub children: ChildrenWithProps<CosmoTableRow>,
}

#[styled_component(CosmoTable)]
pub fn table(props: &CosmoTableProps) -> Html {
    let table_style = use_style!(
        r#"
border-collapse: collapse;

thead tr th {
	font-weight: var(--font-weight-bold);
	padding: var(--table-th-padding-top) var(--table-th-padding-side) var(--table-th-padding-bottom);
	text-transform: capitalize;
	border-bottom: var(--table-th-border-bottom-width) solid var(--primary-color);
	font-family: var(--font-family-heading);
}

tbody tr td {
	border-bottom: var(--table-td-border-bottom-width) solid var(--control-border-color);
	font-weight: var(--font-weight-light);
	padding: var(--table-td-padding-top) var(--table-td-padding-side) var(--table-td-padding-bottom);
}

tr:nth-child(2n-1) td {
    background: var(--table-stripe-color);
}
    "#
    );

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

#[function_component(CosmoTableRow)]
pub fn table_row(props: &CosmoTableRowProps) -> Html {
    html!(
        <tr>
            {for props.children.iter()}
        </tr>
    )
}

impl CosmoTableRow {
    pub fn new(props: CosmoTableRowProps, key: Option<Key>) -> VChild<Self> {
        VChild::new(props, key)
    }

    pub fn from_table_cells(cells: Vec<VChild<CosmoTableCell>>, key: Option<Key>) -> VChild<Self> {
        VChild::new(
            CosmoTableRowProps {
                children: ChildrenRenderer::new(cells),
            },
            key,
        )
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct CosmoTableCellProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CosmoTableCell)]
pub fn table_cell(props: &CosmoTableCellProps) -> Html {
    html!(
        <td>
            {for props.children.iter()}
        </td>
    )
}

impl CosmoTableCell {
    pub fn new(props: CosmoTableCellProps, key: Option<Key>) -> VChild<Self> {
        VChild::new(props, key)
    }

    pub fn from_html(content: Html, key: Option<Key>) -> VChild<Self> {
        VChild::new(
            CosmoTableCellProps {
                children: ChildrenRenderer::new(vec![content]),
            },
            key,
        )
    }
}
