use plotly::common::Title;
use plotly::layout::RelayoutLayout;

use plotly::{
    Histogram,
    common::Visible,
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu},
    },
    Layout, Plot
};

pub fn betti_number_histogram(betti_numbers_vec: &Vec<Vec<i32>>) -> Plot{
    let n = betti_numbers_vec[0].len();
    let mut plot = Plot::new();
    for k in 0..n{
        let mut tmp_betti_numbers: Vec<i32> = Vec::new();
        for betti_numbers in betti_numbers_vec.iter(){
            tmp_betti_numbers.push(*betti_numbers.get(k).unwrap_or_else(|| &0));
        }
        let visible = match k {
            0 => Visible::True,
            _ => Visible::False
        };
        let k_str: &str = &k.to_string();
        let trace = Histogram::new(tmp_betti_numbers).name(k_str).visible(visible);
        plot.add_trace(trace);
    }

    let mut buttons = Vec::new();
    for k in 0..n{
        let k_str: &str = &k.to_string();
        let visibility = (0..n).into_iter().map(|i| if i == k { Visible::True } else { Visible::False }).collect::<Vec<_>>();
        let title = "Betti number: ".to_owned() + k_str;
        let title_str = title.as_str();
        let layout = RelayoutLayout::ModifyTitle { title: Some(Title::new(title_str)) };
        let btn = ButtonBuilder::new()
            .label(k_str)
            .push_restyle(Histogram::<i32>::modify_visible(visibility))
            .push_relayout(layout)
            .build();
        buttons.push(btn);
    }
    let layout = Layout::new().title(Title::new("Betti number: 0")).update_menus(vec![UpdateMenu::new().y(0.8).buttons(buttons)]);
    plot.set_layout(layout);

    plot
}
