pub mod plot {

    use crate::cgr::cgr::CGR;
    use plotters::prelude::*;

    pub fn plot(cgr: CGR, file_name: &str, dim: u32) -> Result<(), Box<dyn std::error::Error>> {
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

        // tried to make a custom iterator for below
        // but it's too much for me.
        let x = cgr.x;
        let y = cgr.y;
        scatter_ctx.draw_series(x.iter().zip(y).map(|(x, y)| Pixel::new((*x, y), &BLACK)))?;
        Ok(())
    }
}
