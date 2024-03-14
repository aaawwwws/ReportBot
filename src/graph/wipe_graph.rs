use std::{cell::RefCell, collections::HashMap, rc::Rc};

use plotters::prelude::*;

pub struct WipeGraph {
    phase: usize,
    area_name: String,
}
impl WipeGraph {
    pub fn new(phase: usize, area_name: &str) -> Self {
        Self {
            phase,
            area_name: String::from(area_name),
        }
    }

    pub fn create_graph(&self, data: &Vec<usize>) -> anyhow::Result<()> {
        const FILE_NAME: &str = "graph.png";
        let mut map: HashMap<usize, u8> = HashMap::new();
        let mut array = Vec::new();
        let new_data = if data.is_empty() {
            array.push(*&self.phase);
            for d in array.iter() {
                if let Some(a) = map.get(d) {
                    map.insert(*d, *a + 1);
                }
                map.insert(*d, 1);
            }
            array
        } else {
            for d in data {
                if let Some(a) = map.get(d) {
                    map.insert(*d, *a + 1);
                }
                map.insert(*d, 1);
            }
            data.clone()
        };
        let max_wipe: u32 = *map.values().max().unwrap() as u32;

        let root = BitMapBackend::new(FILE_NAME, (480, 480)).into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(&self.area_name, ("meiryo", 25).into_font())
            .build_cartesian_2d(
                (1u32..*new_data.iter().max().unwrap() as u32).into_segmented(),
                0u32..max_wipe,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("ワイプ数")
            .x_desc("フェーズ")
            .axis_desc_style(("meiryo", 15).into_font())
            .label_style(("meiryo", 30).into_font())
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(new_data.iter().map(|x| (SegmentValue::from(*x as u32), 1))),
        )?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().unwrap();
        Ok(())
    }

    pub fn get_phase(&self) -> &usize {
        &self.phase
    }
}
