pub mod plot {

    use crate::cgr::cgr::CGR;
    use plotters::prelude::*;

    pub fn plot(
        cgr: CGR,
        file_name: &str,
        dim: u32,
        grid: bool,
        order: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(file_name, (dim, dim)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut scatter_ctx = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_ranged(-1f64..1f64, -1f64..1f64)?;

        scatter_ctx
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        if grid {
            make_grid(&mut scatter_ctx, order);
        }
        // tried to make a custom iterator for below
        // but it's too much for me.
        let x = cgr.x;
        let y = cgr.y;
        scatter_ctx.draw_series(x.iter().zip(y).map(|(x, y)| Pixel::new((*x, y), &BLACK)))?;
        Ok(())
    }

    #[derive(Debug)]
    struct BoxCoords {
        horizontals: Vec<((f64, f64), (f64, f64))>,
        verticals: Vec<((f64, f64), (f64, f64))>,
    }

    fn make_grid(
        context: &mut plotters::chart::ChartContext<
            plotters::drawing::BitMapBackend,
            plotters::coord::RangedCoord<
                plotters::coord::RangedCoordf64,
                plotters::coord::RangedCoordf64,
            >,
        >,
        order: f64,
    ) {
        let x = -1f64;
        let y = 1f64;

        let order = 2f64.powf(order);

        let diff = (y - x) / order;

        let mut x0 = -1f64;
        let mut horizontals = Vec::new();
        let mut verticals = Vec::new();

        loop {
            if x0 > y {
                break;
            } else {
                // horizontals
                horizontals.push(((-1f64, x0), (1f64, x0)));
                // verticals
                verticals.push(((x0, 1f64), (x0, -1f64)));
                x0 += diff;
            }
        }

        horizontals.remove(0);
        verticals.remove(0);
        horizontals.remove(horizontals.len() - 1);
        verticals.remove(verticals.len() - 1);

        // draw horizontals
        for (index, (x, y)) in horizontals.iter().enumerate() {
            if index == (horizontals.len() as f64 / 2 as f64).floor() as usize {
                context
                    .draw_series(LineSeries::new(vec![(x.0, x.1), (y.0, y.1)], &RED))
                    .unwrap();
            } else {
                context
                    .draw_series(LineSeries::new(
                        vec![(x.0, x.1), (y.0, y.1)],
                        &BLACK.mix(0.5),
                    ))
                    .unwrap();
            }
        }
        // draw verticals
        for (index, (x, y)) in verticals.iter().enumerate() {
            if index == (verticals.len() as f64 / 2 as f64).floor() as usize {
                context
                    .draw_series(LineSeries::new(vec![(x.0, x.1), (y.0, y.1)], &RED))
                    .unwrap();
            } else {
                context
                    .draw_series(LineSeries::new(
                        vec![(x.0, x.1), (y.0, y.1)],
                        &BLACK.mix(0.5),
                    ))
                    .unwrap();
            }
        }
    }
}
